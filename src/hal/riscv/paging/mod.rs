use core::arch::asm;
use riscv::register::satp;

pub mod entry;

pub fn activate_virt_mem(token: usize) {
    let satp_token = 8usize << 60 | token;
    satp::write(satp_token);
}
