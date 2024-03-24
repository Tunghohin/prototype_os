use core::arch::asm;
use riscv::register::satp;

pub mod entry;

pub struct TokenSV39 {
    pub bits: usize,
}

impl TokenSV39 {
    fn new(ppn: usize) -> Self {
        Self {
            bits: 8usize << 60 | ppn,
        }
    }
}

pub fn activate_virt_mem(token: usize) {
    unsafe {
        satp::set(satp::Mode::Sv39, 0, token);
    }
}
