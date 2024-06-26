#![no_std]
#![no_main]

extern crate alloc;

mod hal;
mod lang_items;
mod misc;
mod mm;
mod ramfs;
mod sync;
mod sysconfig;
mod task;

use core::arch::global_asm;

global_asm!(include_str!("bootloader.asm"));

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn bss_with_stack();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

fn seg_info() {
    println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    println!(
        ".bss [{:#x}, {:#x})",
        bss_with_stack as usize, ebss as usize
    );
    println!(".ekernel [{:#x}]", ekernel as usize);
    println!(".strampoline [{:#x}]", strampoline as usize);
}

fn bootup_logo() {
    println!(
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
    println!("{:x}, {:x}", sbss as usize, ebss as usize);
    unsafe {
        core::slice::from_raw_parts_mut(sbss as *mut u8, ebss as usize - sbss as usize).fill(0);
    }
}

fn kernel_init() {
    seg_info();
    clear_bss();
    hal::init();
    mm::init();
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    kernel_init();
    bootup_logo();
    task::init();
    task::sche::run_task();
    shut_down();
}

fn shut_down() -> ! {
    panic!("shut down!");
}
