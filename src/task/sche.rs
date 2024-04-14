#![allow(dead_code)]

use core::ptr::drop_in_place;

use crate::hal::*;
use crate::sync::upsafecell::UPSafeCell;
use crate::task::cpu;
use crate::task::cpu::PROCESSOR;
use crate::task::task::TaskControlBlock;
use crate::task::task::TaskStatus;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;

pub struct TaskReadyQueue {
    queue: VecDeque<Arc<TaskControlBlock>>,
}

impl TaskReadyQueue {
    fn new() -> Self {
        TaskReadyQueue {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, new_task: Arc<TaskControlBlock>) {
        self.queue.push_back(new_task);
    }

    fn pop(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.queue.pop_front()
    }

    pub fn len(&mut self) -> usize {
        self.queue.len()
    }
}

lazy_static! {
    pub static ref TASK_QUEUE: UPSafeCell<TaskReadyQueue> =
        unsafe { UPSafeCell::new(TaskReadyQueue::new()) };
}

pub fn add_task(new_task: Arc<TaskControlBlock>) {
    TASK_QUEUE.exclusive_access().push(new_task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_QUEUE.exclusive_access().pop()
}

pub fn run_task() {
    loop {
        if let Some(task) = fetch_task() {
            let mut processor = PROCESSOR.exclusive_access();
            let idle_cx = &mut processor.idle_task_cx as *mut TaskContext;
            let mut task_inner = task.inner_exclusive_access();
            task_inner.status = TaskStatus::Ready;
            let task_cx = (&task_inner.cx) as *const TaskContext;
            drop(task_inner);
            processor.current = Some(task);
            drop(processor);
            TaskContext::switch(idle_cx, task_cx);
        }
    }
}

pub fn suspend_current() {
    let current_task = cpu::take_current_task().expect("No current task.");
    let mut current_task_inner = current_task.inner_exclusive_access();
    let current_task_cx = (&mut current_task_inner.cx) as *mut TaskContext;
    drop(current_task_inner);
    add_task(current_task);
    scheduler(current_task_cx);
}

pub fn scheduler(task_cx: *mut TaskContext) -> ! {
    let processor = PROCESSOR.exclusive_access();
    let idle_cx = (&processor.idle_task_cx) as *const TaskContext;
    drop(processor);
    TaskContext::switch(task_cx, idle_cx);
}
