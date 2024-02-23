use crate::misc::range::StepByOne;
use crate::mm::page_table::entry::PageTableEntry;
use crate::sysconfig::{PAGE_SIZE, PAGE_SIZE_BITS};
use core::fmt::Debug;
use core::mem::size_of;

pub const PA_WIDTH_SV39: usize = 56;
pub const VA_WIDTH_SV39: usize = 39;
pub const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
pub const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

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

impl From<usize> for PhysAddrSV39 {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for PhysPageNumSV39 {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH_SV39) - 1))
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

impl From<PhysAddrSV39> for usize {
    fn from(v: PhysAddrSV39) -> Self {
        v.0
    }
}

impl From<PhysPageNumSV39> for usize {
    fn from(v: PhysPageNumSV39) -> Self {
        v.0
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

impl VirtAddrSV39 {
    /// Get the (floor) virtual page number
    pub fn floor(&self) -> VirtPageNumSV39 {
        VirtPageNumSV39(self.0 / PAGE_SIZE)
    }

    /// Get the (ceil) virtual page number
    pub fn ceil(&self) -> VirtPageNumSV39 {
        VirtPageNumSV39((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }

    /// Get the page offset of virtual address
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    /// Check if the virtual address is aligned by page size
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl From<VirtAddrSV39> for VirtPageNumSV39 {
    fn from(v: VirtAddrSV39) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<VirtPageNumSV39> for VirtAddrSV39 {
    fn from(v: VirtPageNumSV39) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl PhysAddrSV39 {
    /// Get the (floor) physical page number
    pub fn floor(&self) -> PhysPageNumSV39 {
        PhysPageNumSV39(self.0 / PAGE_SIZE)
    }
    /// Get the (ceil) physical page number
    pub fn ceil(&self) -> PhysPageNumSV39 {
        PhysPageNumSV39((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }
    /// Get the page offset of physical address
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    /// Check if the physical address is aligned by page size
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl From<PhysAddrSV39> for PhysPageNumSV39 {
    fn from(v: PhysAddrSV39) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<PhysPageNumSV39> for PhysAddrSV39 {
    fn from(v: PhysPageNumSV39) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl PhysAddrSV39 {
    /// Get the immutable reference of physical address
    pub fn get_ref<T>(&self) -> &'static T {
        unsafe { (self.0 as *const T).as_ref().unwrap() }
    }
    /// Get the mutable reference of physical address
    pub fn get_mut<T>(&self) -> &'static mut T {
        unsafe { (self.0 as *mut T).as_mut().unwrap() }
    }
}

impl PhysPageNumSV39 {
    /// Get the reference of page table(array of ptes)
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddrSV39 = (*self).into();
        unsafe {
            core::slice::from_raw_parts_mut(
                pa.0 as *mut PageTableEntry,
                PAGE_SIZE / size_of::<PageTableEntry>(),
            )
        }
    }
    /// Get the reference of page(array of bytes)
    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddrSV39 = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, 4096) }
    }
    /// Get the mutable reference of physical address
    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddrSV39 = (*self).into();
        pa.get_mut()
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
