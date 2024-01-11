#![no_std]
#![no_main]

extern crate alloc;

mod lang_item;
mod mm;
mod sysconfig;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

fn shut_down() -> ! {
    panic!("Shut down!");
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    mm::init();
    shut_down();
}
