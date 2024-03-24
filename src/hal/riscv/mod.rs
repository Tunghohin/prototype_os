pub mod address;
pub mod console;
pub mod context;
pub mod paging;
pub mod sbi;
pub mod syscall;
pub mod trap;

use self::paging::entry::{MapPermissionSV39, PTEFlagsSV39};
use self::trap::TrapContextRV64;
use crate::hal::generic_address::AddressMetaData;
use crate::hal::generic_paging::PagingMetaData;
use crate::hal::riscv::address::{PhysAddrSV39, PhysPageNumSV39, VirtAddrSV39, VirtPageNumSV39};
use crate::hal::riscv::context::TaskContextRV64;
use crate::hal::riscv::paging::entry::PageTableEntrySV39;
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
    type PageTableEntry = PageTableEntrySV39;
    type PTEFlags = PTEFlagsSV39;
    type MapPermission = MapPermissionSV39;
    type TaskContext = TaskContextRV64;
    type TrapContext = TrapContextRV64;
}

pub fn init() {
    trap::init();
}

pub fn activate_virt_mem(token: usize) {
    paging::activate_virt_mem(token);
}
