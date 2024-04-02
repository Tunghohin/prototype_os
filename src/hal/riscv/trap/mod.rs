use crate::hal::riscv::context::RegistersRV64;
use crate::hal::riscv::paging::TokenSV39;
use crate::task::cpu::current_task_token_ppn;
use crate::{hal::generic_trap::GenericTrap, sysconfig::TRAMPOLINE, sysconfig::TRAP_CONTEXT_BASE};
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode, satp, scause, sstatus, sstatus::set_spp, sstatus::Sstatus, stval, stvec,
};

#[no_mangle]
fn trap_in() -> ! {
    crate::println!("Trap in!");
    loop {}
}

pub fn set_trap_entry_kernel() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

pub fn set_trap_entry_user() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

pub fn init() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

/// Trap handler
#[no_mangle]
pub extern "C" fn trap_handler() {
    set_trap_entry_kernel();
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        _ => {
            panic!(
                "Unsupported: scause: {:?}, stval{:?}",
                scause.cause(),
                stval
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn trap_from_kernel() -> ! {
    let scause = scause::read();
    let stval = stval::read();
    panic!(
        "Trap from kernel is not yet supported!: scause: {:?}, stval: {:?}",
        scause.cause(),
        stval
    );
    panic!("Trap from kernel is not yet supported!");
}

/// Trap context of riscv64
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TrapContextRV64 {
    /// General-Purpose Register x0-31
    pub regs: RegistersRV64,
    /// Supervisor Status Register
    pub sstatus: usize,
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
    fn task_init_cx(entry: usize, user_sp: usize, kernel_sp: usize) -> TrapContextRV64 {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(sstatus::SPP::User);
        let sstatus = sstatus.bits();
        let mut cx = TrapContextRV64 {
            regs: unsafe { core::mem::zeroed::<RegistersRV64>() },
            sstatus,
            sepc: entry,
            kernel_satp: satp::read().bits(),
            kernel_sp,
            trap_handler: trap_handler as usize,
        };
        cx.regs.sp = user_sp;
        cx
    }

    fn init() {}
}

global_asm!(include_str!("trapin.asm"));
#[no_mangle]
pub extern "C" fn trap_return() -> ! {
    set_trap_entry_user();
    extern "C" {
        fn __trapin();
        fn __restore();
    }
    let user_token = TokenSV39::new(current_task_token_ppn()).bits();
    let restore_va = __restore as usize - __trapin as usize + TRAMPOLINE;
    unsafe {
        core::arch::asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") TRAP_CONTEXT_BASE,
            in("a1") user_token,
            options(noreturn)
        );
    }
    panic!("Not supposed to get there.");
}
