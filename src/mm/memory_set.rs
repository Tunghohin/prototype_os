#![allow(dead_code)]

use crate::hal::*;
use crate::misc::range::SimpleRange;
use crate::mm::page_table::frame::frame_alloc;
use crate::mm::page_table::frame::FrameTracker;
use crate::mm::page_table::PageTable;
use crate::sync::upsafecell::UPSafeCell;
use crate::sysconfig::MEMORY_END;
use crate::sysconfig::TRAMPOLINE;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use bitflags::*;
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

bitflags! {
    /// map permission corresponding to that in pte: `R W X U`
    pub struct MapPermission: u8 {
        /// readable
        const R = 1 << 1;
        /// writable
        const W = 1 << 2;
        /// excutable
        const X = 1 << 3;
        /// u-mode accessible
        const U = 1 << 4;
    }
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

    pub fn insert_mapped_segment(seg: MapSegment, data: Option<&mut [u8]>) {}

    pub fn insert_segment(&mut self, mut seg: MapSegment, data: Option<&mut [u8]>) {
        seg.map(&mut self.page_table);
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

        log::info!("kernel memory set:");
        log::info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        log::info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        log::info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        log::info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
        log::info!("mapping .text section");

        log::info!("mapping .text section");
        memory_set.insert_segment(
            MapSegment::new(
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );
        log::info!("mapping .rodata section");
        memory_set.insert_segment(
            MapSegment::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                MapPermission::R,
            ),
            None,
        );
        log::info!("mapping .data section");
        memory_set.insert_segment(
            MapSegment::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        log::info!("mapping .bss section");
        memory_set.insert_segment(
            MapSegment::new(
                (sbss as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        log::info!("mapping physical memory");
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
}

pub struct MapSegment {
    mapping: BTreeMap<VirtPageNum, FrameTracker>,
    map_type: MapType,
    permission: MapPermission,
    vpn_range: VPNRange,
}

impl MapSegment {
    fn new(
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
        let pte_flags = PTEFlags::from_bits(self.permission.bits).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }

    fn map(&mut self, page_table: &mut PageTable) {
        self.vpn_range.clone().into_iter().for_each(|vpn| {
            self.map_one(vpn, page_table);
        })
    }
}
