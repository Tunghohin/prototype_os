use crate::hal::generic_context::GenericContext;
use crate::hal::*;
use crate::sync::upsafecell::UPSafeCell;
use crate::task::task::TaskControlBlock;
use alloc::sync::Arc;
use lazy_static::*;

pub struct CPU {
    pub current: Option<Arc<TaskControlBlock>>,
    pub idle_task_cx: TaskContext,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }
}
