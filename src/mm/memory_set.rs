#![allow(dead_code)]

use core::cmp;

use crate::hal::*;
use crate::misc::range::SimpleRange;
use crate::misc::range::StepByOne;
use crate::mm::page_table::frame::frame_alloc;
use crate::mm::page_table::frame::FrameTracker;
use crate::mm::page_table::PageTable;
use crate::sync::upsafecell::UPSafeCell;
use crate::sysconfig::{MEMORY_END, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT_BASE, USER_STACK_SIZE};
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use lazy_static::*;

type VPNRange = SimpleRange<VirtPageNum>;

lazy_static! {
    /// The kernel's initial memory mapping(kernel address space)
    pub static ref KERNEL_SPACE: Arc<UPSafeCell<MemorySet>> =
        Arc::new(unsafe { UPSafeCell::new(MemorySet::new_kernel()) });
}

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

pub enum MapType {
    Identical,
    Framed,
}

pub struct MemorySet {
    page_table: PageTable,
    segments: Vec<MapSegment>,
}

impl MemorySet {
    pub fn new() -> MemorySet {
        Self {
            page_table: PageTable::new(),
            segments: Vec::new(),
        }
    }

    pub fn insert_segment(&mut self, mut seg: MapSegment, data: Option<&[u8]>) {
        seg.map(&mut self.page_table);
        if let Some(data) = data {
            let mut current_vpn: VirtPageNum = seg.vpn_range.get_start().into();
            let mut current_read: usize = 0;
            let mut remain = data.len();
            while remain != 0 {
                let read_size = core::cmp::min(PAGE_SIZE, remain);
                let src = &data[current_read..current_read + read_size];
                let dst = self
                    .page_table
                    .translate_ppn(current_vpn.into())
                    .get_bytes_array_mut();
                dst.copy_from_slice(src);
                remain -= read_size;
                current_vpn.step();
                current_read += read_size;
            }
        }
        self.segments.push(seg);
    }

    fn map_trampoline(&mut self) {
        self.page_table.map(
            VirtAddr::from(TRAMPOLINE).into(),
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X,
        );
    }

    /// only run it on kernel space
    pub fn activate(&self) {
        activate_virt_mem(self.page_table.root_ppn.into());
    }

    pub fn new_kernel() -> MemorySet {
        let mut memory_set = MemorySet::new();
        memory_set.map_trampoline();

        memory_set.insert_segment(
            MapSegment::new(
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );
        memory_set.insert_segment(
            MapSegment::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                MapPermission::R,
            ),
            None,
        );
        memory_set.insert_segment(
            MapSegment::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        memory_set.insert_segment(
            MapSegment::new(
                (sbss as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        memory_set.insert_segment(
            MapSegment::new(
                (ekernel as usize).into(),
                MEMORY_END.into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        memory_set
    }

    /// return (MemorySet, uset_stack_top, entry_point)
    pub fn new_task(data: &[u8]) -> (MemorySet, usize, usize) {
        let mut memory_set = MemorySet::new();
        memory_set.map_trampoline();

        let elf = xmas_elf::ElfFile::new(data).expect("Invalid ELF data!");
        assert!(
            elf.header.pt1.magic == [0x7f, 0x45, 0x4c, 0x46],
            "Invalid ELF data!"
        );
        let program_header_count = elf.header.pt2.ph_count();
        let mut max_vpn: VirtPageNum = VirtPageNum::from(0);
        for i in 0..program_header_count {
            let program_header = elf.program_header(i).unwrap();

            match program_header.get_type().unwrap() {
                xmas_elf::program::Type::Load => {
                    let mut map_permission = MapPermission::U;
                    let program_header_flag = program_header.flags();

                    if program_header_flag.is_read() {
                        map_permission |= MapPermission::R;
                    }
                    if program_header_flag.is_write() {
                        map_permission |= MapPermission::W;
                    }
                    if program_header_flag.is_execute() {
                        map_permission |= MapPermission::X;
                    }

                    let start_addr: VirtAddr = (program_header.virtual_addr() as usize).into();
                    let end_addr: VirtAddr =
                        ((program_header.virtual_addr() + program_header.mem_size()) as usize)
                            .into();
                    memory_set.insert_segment(
                        MapSegment::new(start_addr, end_addr, MapType::Framed, map_permission),
                        Some(
                            &elf.input[program_header.offset() as usize
                                ..(program_header.offset() + program_header.file_size()) as usize],
                        ),
                    );
                    max_vpn = cmp::max(max_vpn, end_addr.pagenum_ceil());
                }
                _ => {
                    continue;
                }
            }
        }
        let mut user_stack_bottom: usize = VirtAddr::from(max_vpn).into();
        user_stack_bottom += PAGE_SIZE;
        let user_stack_top = user_stack_bottom + USER_STACK_SIZE;
        // map user stack
        memory_set.insert_segment(
            MapSegment::new(
                user_stack_bottom.into(),
                user_stack_top.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W | MapPermission::U,
            ),
            None,
        );
        // map trap context
        memory_set.insert_segment(
            MapSegment::new(
                TRAP_CONTEXT_BASE.into(),
                TRAMPOLINE.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        (
            memory_set,
            user_stack_top,
            elf.header.pt2.entry_point() as usize,
        )
    }
}

pub struct MapSegment {
    mapping: BTreeMap<VirtPageNum, FrameTracker>,
    map_type: MapType,
    permission: MapPermission,
    vpn_range: VPNRange,
}

impl MapSegment {
    pub fn new(
        start_vaddr: VirtAddr,
        end_vaddr: VirtAddr,
        map_type: MapType,
        permission: MapPermission,
    ) -> MapSegment {
        let start_vpn = start_vaddr.pagenum_floor();
        let end_vpn = end_vaddr.pagenum_ceil();
        Self {
            mapping: BTreeMap::new(),
            map_type,
            permission,
            vpn_range: VPNRange::new(start_vpn, end_vpn),
        }
    }

    fn map_one(&mut self, vpn: VirtPageNum, page_table: &mut PageTable) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => {
                ppn = vpn.0.into();
            }
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.mapping.insert(vpn, frame);
            }
        }
        let pte_flags = PTEFlags::from_bits(self.permission.bits()).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }

    fn map(&mut self, page_table: &mut PageTable) {
        self.vpn_range.clone().into_iter().for_each(|vpn| {
            self.map_one(vpn, page_table);
        })
    }
}

pub fn remap_test() {
    let kernel_space = KERNEL_SPACE.exclusive_access();
    let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_rodata: VirtAddr = ((srodata as usize + erodata as usize) / 2).into();
    let mid_data: VirtAddr = ((sdata as usize + edata as usize) / 2).into();
    assert_eq!(
        kernel_space
            .page_table
            .translate_pte(mid_text.into())
            .unwrap()
            .is_readable(),
        true
    );
    assert_eq!(
        kernel_space
            .page_table
            .translate_pte(mid_text.into())
            .unwrap()
            .is_executable(),
        true
    );
    assert_eq!(
        kernel_space
            .page_table
            .translate_pte(mid_text.into())
            .unwrap()
            .is_writable(),
        false
    );
    assert_eq!(
        kernel_space
            .page_table
            .translate_pte(mid_data.into())
            .unwrap()
            .is_writable(),
        true
    );
    assert_eq!(
        kernel_space
            .page_table
            .translate_pte(mid_data.into())
            .unwrap()
            .is_readable(),
        true
    );
    assert_eq!(
        kernel_space
            .page_table
            .translate_pte(mid_data.into())
            .unwrap()
            .is_executable(),
        false
    );
    assert_eq!(
        kernel_space
            .page_table
            .translate_pte(mid_rodata.into())
            .unwrap()
            .is_readable(),
        true
    );
    assert_eq!(
        kernel_space
            .page_table
            .translate_pte(mid_rodata.into())
            .unwrap()
            .is_executable(),
        false
    );
    assert_eq!(
        kernel_space
            .page_table
            .translate_pte(mid_rodata.into())
            .unwrap()
            .is_writable(),
        false
    );
    crate::println!("[kernel testing] remap_test passed!");
}
