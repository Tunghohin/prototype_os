use crate::print;
use crate::task::cpu::current_task;

const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let buffers = current_task()
                .expect("No current task")
                .inner_exclusive_access()
                .memory_set
                .translate_bytes_buffer(buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
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
