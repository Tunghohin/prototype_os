use core::cell::RefMut;

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

    pub fn take_current_task(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    pub fn current_task(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_mut().cloned()
    }

    pub fn current_task_token_ppn(&mut self) -> usize {
        self.current_task()
            .expect("No current task!")
            .inner_exclusive_access()
            .memory_set
            .get_root_ppn()
            .0
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}
