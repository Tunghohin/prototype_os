#![no_std]
#![no_main]

extern crate alloc;

mod heap_allocator;
mod lang_item;
mod sysconfig;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    heap_allocator::init();
    loop {}
}
