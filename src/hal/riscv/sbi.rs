use core::arch::asm;

/// console putchar sbi call id
const SBI_CONSOLE_PUTCHAR: usize = 1;
/// console getchar sbi call id
const SBI_CONSOLE_GETCHAR: usize = 2;

/// general sbi call
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "li x16, 0", // for sbi call id args need 2 reg (x16, x17)
            "ecall",     // sbi call
            inlateout("x10") arg0 => ret, // sbi call arg0 and return value
            in("x11") arg1, // sbi call arg1
            in("x12") arg2, // sbi call arg2
            in("x17") which,// sbi call id
        );
    }
    ret
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

/// use sbi call to set timer
pub fn set_timer(timer: usize) {
    sbi_rt::set_timer(timer as _);
}

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
