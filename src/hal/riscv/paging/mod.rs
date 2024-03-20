use core::arch::asm;
use riscv::register::satp;

pub mod entry;

pub fn activate_virt_mem(token: usize) {
    unsafe {
        satp::set(satp::Mode::Sv39, 0, token);
    }
}
