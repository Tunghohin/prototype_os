use {
    core::arch::global_asm,
    riscv::register::{mtvec::TrapMode, stvec},
};

global_asm!(include_str!("trapin.asm"));

pub fn init() {
    extern "C" {
        fn __trapin();
    }
    unsafe {
        stvec::write(__trapin as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub extern "C" fn trap_handler() {}
