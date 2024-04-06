mod fs;
mod proc;
mod timer;

use crate::hal::riscv::syscall::fs::{sys_read, sys_write};

use self::proc::sys_exit;

const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        _ => {
            panic!("Unsupported syscall: ID = {}", syscall_id);
        }
    }
}

pub fn set_next_trigger() {
    timer::set_next_trigger();
}
