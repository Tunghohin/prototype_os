pub mod cpu;
pub mod pid;
pub mod sche;
pub mod task;

use crate::task::sche::add_task;

pub fn init() {
    add_task(task::INITPROC.clone());
}
