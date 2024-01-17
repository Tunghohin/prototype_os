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

#[derive(Debug, Copy, Clone), repr(C)]
