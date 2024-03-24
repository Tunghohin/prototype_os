#![no_std]
#![no_main]

extern crate alloc;

mod hal;
mod lang_items;
mod loader;
mod misc;
mod mm;
mod sync;
mod sysconfig;
mod task;

use core::arch::global_asm;

global_asm!(include_str!("bootloader.asm"));

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
    hal::init();
    mm::init();
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    kernel_init();
    bootup_logo();
    crate::task::sche::add_task(crate::task::task::INITPROC.clone());
    crate::task::sche::add_task(crate::task::task::INITPROC.clone());
    crate::task::sche::add_task(crate::task::task::INITPROC.clone());
    crate::task::sche::add_task(crate::task::task::INITPROC.clone());
    println!("{}", crate::task::sche::TASK_QUEUE.exclusive_access().len());
    shut_down();
}

fn shut_down() -> ! {
    panic!("shut down!");
}
