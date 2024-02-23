use crate::hal::*;
use crate::misc::range::SimpleRange;
use crate::mm::page_table::frame::FrameTracker;
use crate::mm::page_table::PageTable;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use bitflags::*;

type VPNRange = SimpleRange<VirtPageNum>;

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
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

struct MemorySet {
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

    pub fn new_kernel() -> MemorySet {
        let mut memory_set = MemorySet::new();

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

    fn map_one(&self, vpn: VirtPageNum, page_table: &mut PageTable) {
        match self.map_type {
            MapType::Identical => {}
            MapType::Framed => {}
        }
    }

    fn map(&mut self, page_table: &mut PageTable) {
        self.vpn_range.clone().into_iter().for_each(|vpn| {
            self.map_one(vpn, page_table);
        })
    }
}
