#![no_std]
#![no_main]

extern crate alloc;

mod hal;
mod lang_items;
mod misc;
mod mm;
mod sync;
mod sysconfig;
mod task;

use core::arch::global_asm;

fn bootup_logo() {
    print!(
        r"
__________                __          __                        ________    _________
\______   \_______  _____/  |_  _____/  |_ ___.__.______   ____ \_____  \  /   _____/
 |     ___/\_  __ \/  _ \   __\/  _ \   __<   |  |\____ \_/ __ \ /   |   \ \_____  \ 
 |    |     |  | \(  <_> )  | (  <_> )  |  \___  ||  |_> >  ___//    |    \/        \
 |____|     |__|   \____/|__|  \____/|__|  / ____||   __/ \___  >_______  /_______  /
                                           \/     |__|        \/        \/        \/ 
        "
    );
}

/// clear bss
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as *mut u8, ebss as usize - sbss as usize).fill(0);
    }
}

fn kernel_init() {
    clear_bss();
    misc::logger::init();
    hal::init();
    mm::init();
}

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    kernel_init();
    bootup_logo();
    shut_down();
}

fn shut_down() -> ! {
    panic!("shut down!");
}
