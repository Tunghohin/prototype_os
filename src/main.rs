#![no_std]
#![no_main]

extern crate alloc;

mod hal;
mod lang_items;
mod mm;
mod sync;
mod sysconfig;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

pub fn bootup_logo() {
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

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    mm::init();
    hal::init();

    bootup_logo();

    shut_down();
}

fn shut_down() -> ! {
    loop {}
}
