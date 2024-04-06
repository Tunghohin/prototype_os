use crate::mm::page_table::page_table::translate_bytes_buffer;
use crate::task::cpu::current_task;
use crate::{print, println};

const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let buffers = translate_bytes_buffer(buf, len);
            for buffer in buffers {
                unsafe {
                    print!("{}", core::str::from_utf8_unchecked(buffer));
                }
            }
            return len as isize;
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
    0
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    0
}
