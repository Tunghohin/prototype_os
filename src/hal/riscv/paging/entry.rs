use crate::hal::riscv::address::{PhysPageNumSV39, PPN_WIDTH_SV39};
use crate::hal::{generic_paging::GenericPTEFlag, generic_paging::GenericPagetableEntry};
use bitflags::*;

bitflags! {
    /// page table entry flags
    pub struct PTEFlagsSV39: u8 {
        /// validity
        const V = 1 << 0;
        /// readable
        const R = 1 << 1;
        /// writable
        const W = 1 << 2;
        /// executable
        const X = 1 << 3;
        /// u-mode accessibla
        const U = 1 << 4;
        /// global mapping
        const G = 1 << 5;
        /// accessed
        const A = 1 << 6;
        /// dirty
        const D = 1 << 7;
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
/// pagetable entry
pub struct PageTableEntrySV39 {
    pub bits: usize,
}

impl GenericPTEFlag for PTEFlagsSV39 {}

impl GenericPagetableEntry<PTEFlagsSV39> for PageTableEntrySV39 {
    /// create a new pagetable entry
    fn new(ppn: PhysPageNumSV39, flags: PTEFlagsSV39) -> Self {
        PageTableEntrySV39 {
            bits: (ppn.0 << 10) | flags.bits as usize,
        }
    }
    /// validity
    fn is_valid(&self) -> bool {
        (self.bits & PTEFlagsSV39::V.bits as usize) != 0
    }
    /// readable
    fn is_readable(&self) -> bool {
        (self.bits & PTEFlagsSV39::R.bits as usize) != 0
    }
    /// writable
    fn is_writable(&self) -> bool {
        (self.bits & PTEFlagsSV39::R.bits as usize) != 0
    }
    /// executable
    fn is_executable(&self) -> bool {
        (self.bits & PTEFlagsSV39::X.bits as usize) != 0
    }
    /// u-mode accessibla
    fn is_uaccessible(&self) -> bool {
        (self.bits & PTEFlagsSV39::V.bits as usize) != 0
    }
    /// global mapping
    fn is_gmapping(&self) -> bool {
        (self.bits & PTEFlagsSV39::G.bits as usize) != 0
    }
    /// accessed
    fn is_accessed(&self) -> bool {
        (self.bits & PTEFlagsSV39::A.bits as usize) != 0
    }
    /// dirty
    fn is_dirty(&self) -> bool {
        (self.bits & PTEFlagsSV39::D.bits as usize) != 0
    }

    fn get_ppn(&self) -> PhysPageNumSV39 {
        ((self.bits >> 10) & ((1 << PPN_WIDTH_SV39) - 1)).into()
    }
}
