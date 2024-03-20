use crate::hal::*;
use crate::sync::upsafecell::UPSafeCell;
use crate::task::task::TaskControlBlock;
use alloc::sync::Arc;
use lazy_static::*;

pub struct Processor {
    pub current: Option<Arc<TaskControlBlock>>,
    pub idle_task_cx: TaskContext,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}
