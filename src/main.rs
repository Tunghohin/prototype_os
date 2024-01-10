#![no_std]
#![no_main]

mod lang_item;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    loop {}
}
