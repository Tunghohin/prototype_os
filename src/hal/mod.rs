pub mod generic_address;
pub mod riscv;

use crate::hal::riscv::*;

pub trait AddressMetaData {
    const PA_WIDTH: usize;
    const VA_WIDTH: usize;
    const PPN_WIDTH: usize;
    const VPN_WIDTH: usize;
}

pub trait PagingMetaData {
    const LEVEL: usize;
}

pub trait ArchMetaData: AddressMetaData + PagingMetaData + Sized {}

pub trait GenericArch: ArchMetaData {
    type VPNRange;
    type VirtAddr;
    type VirtPageNum;
    type PhysAddr;
    type PhysPageNum;
}

pub type VPNRange = <ArchRISCV as GenericArch>::VPNRange;
pub type VirtAddr = <ArchRISCV as GenericArch>::VirtAddr;
pub type VirtPageNum = <ArchRISCV as GenericArch>::VirtPageNum;
pub type PhysAddr = <ArchRISCV as GenericArch>::PhysAddr;
pub type PhysPageNum = <ArchRISCV as GenericArch>::PhysPageNum;

pub fn init() {
    riscv::init();
}
