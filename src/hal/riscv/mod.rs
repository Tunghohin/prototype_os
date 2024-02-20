pub mod address;
pub mod console;
pub mod context;
pub mod paging;
pub mod sbi;
pub mod trap;

use crate::hal::generic_address::AddressMetaData;
use crate::hal::generic_paging::PagingMetaData;
use crate::hal::riscv::address::{PhysAddrSV39, PhysPageNumSV39, VirtAddrSV39, VirtPageNumSV39};
use crate::hal::{ArchMetaData, GenericArch};
use crate::sysconfig::PAGE_SIZE_BITS;

/// sv39 specific
pub struct ArchRISCV;

impl AddressMetaData for ArchRISCV {
    const PA_WIDTH: usize = 56;
    const VA_WIDTH: usize = 39;
    const PPN_WIDTH: usize = Self::PA_WIDTH - PAGE_SIZE_BITS;
    const VPN_WIDTH: usize = Self::VA_WIDTH - PAGE_SIZE_BITS;
}

impl PagingMetaData for ArchRISCV {
    const LEVEL: usize = 3;
}

impl ArchMetaData for ArchRISCV {}

impl GenericArch for ArchRISCV {
    type VirtAddr = VirtAddrSV39;
    type VirtPageNum = VirtPageNumSV39;
    type PhysAddr = PhysAddrSV39;
    type PhysPageNum = PhysPageNumSV39;
}

pub fn init() {
    trap::init();
}
