use crate::hal::*;
use crate::misc::range::StepByOne;
use crate::sysconfig::{PAGE_SIZE, PAGE_SIZE_BITS};
use core::fmt::Debug;
use core::mem::size_of;

pub const PA_WIDTH_SV39: usize = 56;
pub const VA_WIDTH_SV39: usize = 39;
pub const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
pub const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

pub const PTEIDX_MASK_SV39: usize = 0x01ff;
pub const PTEIDX_MASK_WIDTH_SV30: usize = 9;

/// Physical Address
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PhysAddrSV39(pub usize);

/// Virtual Address
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VirtAddrSV39(pub usize);

/// Physical Page Number PPN
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PhysPageNumSV39(pub usize);

/// Virtual Page Number VPN
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VirtPageNumSV39(pub usize);

impl GenericAddress for PhysAddrSV39 {
    fn offset(&self) -> usize {
        self.0 & PAGE_SIZE - 1
    }

    fn is_aligned(&self) -> bool {
        self.0 == 0
    }
}

impl GenericPhysAddress for PhysAddrSV39 {
    fn pagenum_floor(&self) -> PhysPageNum {
        PhysPageNumSV39(self.0 / PAGE_SIZE)
    }
    fn pagenum_ceil(&self) -> PhysPageNum {
        PhysPageNumSV39((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }
}

impl From<usize> for PhysAddrSV39 {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<PhysAddrSV39> for usize {
    fn from(v: PhysAddrSV39) -> Self {
        v.0
    }
}

impl From<PhysAddrSV39> for PhysPageNumSV39 {
    fn from(v: PhysAddrSV39) -> Self {
        assert_eq!(v.offset(), 0);
        v.pagenum_floor()
    }
}

impl From<PhysPageNumSV39> for PhysAddrSV39 {
    fn from(v: PhysPageNumSV39) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl GenericPageNum for PhysPageNumSV39 {}

impl GenericPhysPageNum for PhysPageNumSV39 {
    fn get_pte_array(&self) -> &'static [crate::hal::PageTableEntry] {
        let pa: PhysAddrSV39 = (*self).into();
        unsafe {
            core::slice::from_raw_parts_mut(
                pa.0 as *mut PageTableEntry,
                PAGE_SIZE / size_of::<PageTableEntry>(),
            )
        }
    }

    fn get_pte_array_mut(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddrSV39 = (*self).into();
        unsafe {
            core::slice::from_raw_parts_mut(
                pa.0 as *mut PageTableEntry,
                PAGE_SIZE / size_of::<PageTableEntry>(),
            )
        }
    }

    fn get_bytes_array(&self) -> &'static [u8] {
        let pa: PhysAddrSV39 = (*self).into();
        unsafe { core::slice::from_raw_parts(pa.0 as *mut u8, PAGE_SIZE) }
    }

    fn get_bytes_array_mut(&self) -> &'static mut [u8] {
        let pa: PhysAddrSV39 = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, PAGE_SIZE) }
    }
}

impl From<usize> for PhysPageNumSV39 {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<PhysPageNumSV39> for usize {
    fn from(v: PhysPageNumSV39) -> Self {
        v.0
    }
}

impl GenericAddress for VirtAddrSV39 {
    fn offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    fn is_aligned(&self) -> bool {
        self.0 == 0
    }
}

impl GenericVirtAddress for VirtAddrSV39 {
    fn pagenum_floor(&self) -> crate::hal::VirtPageNum {
        VirtPageNumSV39(self.0 / PAGE_SIZE)
    }
    fn pagenum_ceil(&self) -> crate::hal::VirtPageNum {
        VirtPageNumSV39((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }
}

impl GenericPageNum for VirtPageNumSV39 {}

impl GenericVirtPageNum for VirtPageNumSV39 {
    fn get_pte_index(&self, level: usize) -> usize {
        assert!(level < 3);
        (self.0 & (PTEIDX_MASK_SV39 << (level * PTEIDX_MASK_WIDTH_SV30)))
            >> (level * PTEIDX_MASK_WIDTH_SV30)
    }
}

impl From<usize> for VirtAddrSV39 {
    fn from(v: usize) -> Self {
        Self(v & ((1 << VA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtPageNumSV39 {
    fn from(v: usize) -> Self {
        Self(v & ((1 << VPN_WIDTH_SV39) - 1))
    }
}

impl From<VirtAddrSV39> for usize {
    fn from(v: VirtAddrSV39) -> Self {
        if v.0 >= (1 << (VA_WIDTH_SV39 - 1)) {
            v.0 | (!((1 << VA_WIDTH_SV39) - 1))
        } else {
            v.0
        }
    }
}

impl From<VirtPageNumSV39> for usize {
    fn from(v: VirtPageNumSV39) -> Self {
        v.0
    }
}

impl From<VirtAddrSV39> for VirtPageNumSV39 {
    fn from(v: VirtAddrSV39) -> Self {
        v.pagenum_floor()
    }
}
impl From<VirtPageNumSV39> for VirtAddrSV39 {
    fn from(v: VirtPageNumSV39) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl StepByOne for VirtPageNumSV39 {
    fn step(&mut self) {
        self.0 += 1;
    }
}

impl StepByOne for PhysPageNumSV39 {
    fn step(&mut self) {
        self.0 += 1;
    }
}
