#![no_std]
#![no_main]

extern crate alloc;

mod lang_item;
mod mm;
mod sysconfig;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    mm::heap_allocator::init();
    loop {}
}
