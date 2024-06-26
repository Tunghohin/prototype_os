#![allow(unused)]

pub mod generic_address;
pub mod generic_context;
pub mod generic_paging;
pub mod generic_trap;

pub use crate::hal::generic_address::*;
pub use crate::hal::generic_paging::*;

pub use crate::hal::riscv::*;

pub mod riscv;
pub mod x86_64;

pub use generic_address::*;
pub use generic_context::*;
pub use generic_paging::*;
pub use generic_trap::*;

pub type Arch = ArchRISCV;

pub fn init() {
    riscv::init();
}

pub fn enable_timer_interrupt() {
    riscv::trap::enable_timer_interrupt();
}

pub trait ArchMetaData: AddressMetaData + PagingMetaData + Sized {}

pub trait GenericArch: ArchMetaData {
    type VirtAddr;
    type VirtPageNum;
    type PhysAddr;
    type PhysPageNum;
    type PageTableEntry;
    type PTEFlags;
    type MapPermission;
    type TaskContext;
    type TrapContext;
}

pub type VirtAddr = <Arch as GenericArch>::VirtAddr;
pub type VirtPageNum = <Arch as GenericArch>::VirtPageNum;
pub type PhysAddr = <Arch as GenericArch>::PhysAddr;
pub type PhysPageNum = <Arch as GenericArch>::PhysPageNum;
pub type PageTableEntry = <Arch as GenericArch>::PageTableEntry;
pub type PTEFlags = <Arch as GenericArch>::PTEFlags; // Should be implemented by bitflags
pub type MapPermission = <Arch as GenericArch>::MapPermission;
pub type TaskContext = <Arch as GenericArch>::TaskContext;
pub type TrapContext = <Arch as GenericArch>::TrapContext;
