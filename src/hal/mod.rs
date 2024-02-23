pub mod generic_address;
pub mod generic_paging;
pub mod riscv;

pub use crate::hal::generic_address::*;
pub use crate::hal::generic_paging::*;

pub use crate::hal::riscv::*;

pub trait ArchMetaData: AddressMetaData + PagingMetaData + Sized {}

pub trait GenericArch: ArchMetaData {
    type VirtAddr;
    type VirtPageNum;
    type PhysAddr;
    type PhysPageNum;
    type PageTableEntry;
    type PTEFlags;
}

pub type Arch = ArchRISCV;

pub type VirtAddr = <Arch as GenericArch>::VirtAddr;
pub type VirtPageNum = <Arch as GenericArch>::VirtPageNum;
pub type PhysAddr = <Arch as GenericArch>::PhysAddr;
pub type PhysPageNum = <Arch as GenericArch>::PhysPageNum;
pub type PageTableEntry = <Arch as GenericArch>::PageTableEntry;
pub type PTEFlags = <Arch as GenericArch>::PTEFlags;

pub fn init() {
    riscv::init();
}
