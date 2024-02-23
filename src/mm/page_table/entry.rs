#![allow(dead_code)]

use crate::hal::*;
use bitflags::*;

pub const PTEIDX_MASK_SV39: usize = 0x01ff;
pub const PTEIDX_OFFSET_SV39: usize = 12;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
/// pagetable entry
pub struct PageTableEntry {
    bits: usize,
}

impl PageTableEntry {
    /// create a new pagetable entry
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits() as usize,
        }
    }
    /// validity
    pub fn is_valid(&self) -> bool {
        (self.bits & PTEFlags::V.bits() as usize) != 0
    }
    /// readable
    pub fn is_readable(&self) -> bool {
        (self.bits & PTEFlags::R.bits() as usize) != 0
    }
    /// writable
    pub fn is_writable(&self) -> bool {
        (self.bits & PTEFlags::R.bits() as usize) != 0
    }
    /// executable
    pub fn is_executable(&self) -> bool {
        (self.bits & PTEFlags::X.bits() as usize) != 0
    }
    /// u-mode accessibla
    pub fn is_uaccessible(&self) -> bool {
        (self.bits & PTEFlags::V.bits() as usize) != 0
    }
    /// global mapping
    pub fn is_gmapping(&self) -> bool {
        (self.bits & PTEFlags::G.bits() as usize) != 0
    }
    /// accessed
    pub fn is_accessed(&self) -> bool {
        (self.bits & PTEFlags::A.bits() as usize) != 0
    }
    /// dirty
    pub fn is_dirty(&self) -> bool {
        (self.bits & PTEFlags::D.bits() as usize) != 0
    }

    pub fn get_ppn(&self) -> PhysPageNum {
        ((self.bits >> 10) & Arch::PPN_WIDTH).into()
    }
}
