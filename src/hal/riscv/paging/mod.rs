use riscv::register::satp;

pub mod entry;

pub fn activate_virt_mem(token: usize) {
    let satp_token = 0b1000 << 60 | token;
    satp::write(satp_token);
}
