use crate::hal::*;
use core::fmt::Debug;

pub trait AddressMetaData {
    const PA_WIDTH: usize;
    const VA_WIDTH: usize;
    const PPN_WIDTH: usize;
    const VPN_WIDTH: usize;
}

pub trait GenericAddress: Copy + Clone + Ord + Eq + Debug + From<usize> + Into<usize> {
    /// Get address offset
    fn offset(&self) -> usize;
    /// Get whether the address is aligned
    fn is_aligned(&self) -> bool;
}

pub trait GenericPhysAddress: GenericAddress {
    /// Get PhyPageNum which is floored by page size
    fn pagenum_floor(&self) -> PhysPageNum;
    /// Get PhyPageNum which is ceiled by page size
    fn pagenum_ceil(&self) -> PhysPageNum;
    /// Get the immutable reference of physical address
    fn get_ref<T>(&self) -> &'static T {
        let addr: usize = self.clone().into();
        unsafe { (addr as *const T).as_ref().unwrap() }
    }
    /// Get the mutable reference of physical address
    fn get_mut<T>(&self) -> &'static mut T {
        let addr: usize = self.clone().into();
        unsafe { (addr as *mut T).as_mut().unwrap() }
    }
}

pub trait GenericVirtAddress: GenericAddress {
    /// Get VirtPageNum which is floored by page size
    fn pagenum_floor(&self) -> VirtPageNum;
    /// Get VirtPageNum which is ceiled by page size
    fn pagenum_ceil(&self) -> VirtPageNum;
}

pub trait GenericPageNum: Copy + Clone + Ord + Eq + Debug + From<usize> + Into<usize> {}

pub trait GenericPhysPageNum: GenericPageNum + Into<PhysAddr> {
    /// Get the reference of page table(array of ptes)
    fn get_pte_array(&self) -> &'static [PageTableEntry];
    /// Get the reference of page(array of bytes)
    fn get_bytes_array(&self) -> &'static [u8];
    /// Get the mutable reference of page table(array of ptes)
    fn get_pte_array_mut(&self) -> &'static mut [PageTableEntry];
    /// Get the mutable reference of page(array of bytes)
    fn get_bytes_array_mut(&self) -> &'static mut [u8];
    /// Get the immutable reference of physical address
    fn get_ref<T>(&self) -> &'static T {
        let pa: PhysAddr = self.clone().into();
        pa.get_ref()
    }
    /// Get the mutable reference of physical address
    fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddr = self.clone().into();
        pa.get_mut()
    }
}

pub trait GenericVirtPageNum: GenericPageNum + Into<VirtAddr> {
    /// Get page table index by level
    fn get_pte_index(&self, level: usize) -> usize;
}
