#![allow(dead_code)]

use crate::mm::address::PhysPageNum;
use bitflags::*;

bitflags! {
    /// page table entry flags
    pub struct PTEFlags: u8 {
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
pub struct PageTableEntry {
    bits: usize,
}

impl PageTableEntry {
    /// create a new pagetable entry
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }
    /// validity
    pub fn is_valid(&self) -> bool {
        (self.bits & PTEFlags::V.bits as usize) != 0
    }
    /// readable
    pub fn is_readable(&self) -> bool {
        (self.bits & PTEFlags::R.bits as usize) != 0
    }
    /// writable
    pub fn is_writable(&self) -> bool {
        (self.bits & PTEFlags::R.bits as usize) != 0
    }
    /// executable
    pub fn is_executable(&self) -> bool {
        (self.bits & PTEFlags::X.bits as usize) != 0
    }
    /// u-mode accessibla
    pub fn is_uaccessible(&self) -> bool {
        (self.bits & PTEFlags::V.bits as usize) != 0
    }
    /// global mapping
    pub fn is_gmapping(&self) -> bool {
        (self.bits & PTEFlags::G.bits as usize) != 0
    }
    /// accessed
    pub fn is_accessed(&self) -> bool {
        (self.bits & PTEFlags::A.bits as usize) != 0
    }
    /// dirty
    pub fn is_dirty(&self) -> bool {
        (self.bits & PTEFlags::D.bits as usize) != 0
    }
}
