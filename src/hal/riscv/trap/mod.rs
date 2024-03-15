use crate::hal::generic_trap::GenericTrap;
use crate::hal::riscv::context::RegistersRV64;
use core::arch::global_asm;
use riscv::register::{mtvec::TrapMode, satp, sstatus, sstatus::set_spp, sstatus::Sstatus, stvec};

global_asm!(include_str!("trapin.asm"));

extern "C" {
    fn __trapin();
}

pub fn set_trap_entry_kernel() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

pub fn set_trap_entry_user() {
    unsafe {
        stvec::write(__trapin as usize, TrapMode::Direct);
    }
}

pub fn init() {}

#[no_mangle]
pub extern "C" fn trap_handler() {}

#[no_mangle]
pub extern "C" fn trap_from_kernel() -> ! {
    panic!("Trap from kernel is not yet supported!");
}

#[repr(C)]
pub struct TrapContextRV64 {
    /// General-Purpose Register x0-31
    pub register: RegistersRV64,
    /// Supervisor Status Register
    pub sstatus: Sstatus,
    /// Supervisor Exception Program Counter
    pub sepc: usize,
    /// Token of kernel address space
    pub kernel_satp: usize,
    /// Kernel stack pointer of the current application
    pub kernel_sp: usize,
    /// Virtual address of trap handler entry point in kernel
    pub trap_handler: usize,
}

impl GenericTrap<TrapContextRV64> for TrapContextRV64 {
    fn task_init() -> TrapContextRV64 {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(sstatus::SPP::User);
        unsafe { core::mem::zeroed::<TrapContextRV64>() }
    }
}
