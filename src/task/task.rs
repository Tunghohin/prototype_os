#![allow(dead_code)]

use crate::hal::generic_context::GenericContext;
use crate::hal::*;
use crate::mm::memory_set::{MapSegment, MapType, MemorySet, KERNEL_SPACE};
use crate::sync::upsafecell::UPSafeCell;
use crate::task::pid::{kstack_alloc, pid_alloc};
use crate::task::pid::{KernelStack, PidHandle};
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use core::cell::RefMut;

pub struct TaskControlBlock {
    pub pid: PidHandle,
    pub kstack: KernelStack,
    inner: UPSafeCell<TaskControlBlockInner>,
}

impl TaskControlBlock {
    pub fn new(elf_data: &[u8]) -> Self {
        let (memory_set, user_stack_top, entry_point) = MemorySet::new_task(elf_data);
        let pid = pid_alloc();
        let (kstack, kstack_bottom, kstack_top) = kstack_alloc();
        KERNEL_SPACE.exclusive_access().insert_segment(
            MapSegment::new(
                kstack_bottom.into(),
                kstack_top.into(),
                MapType::Framed,
                MapPermission::W | MapPermission::R,
            ),
            None,
        );
        let cx = TaskContext::goto_trap_return(kstack_top);
        TaskControlBlock {
            pid,
            kstack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    status: TaskStatus::Ready,
                    cx,
                    memory_set,
                    exit_code: 0,
                    parent: None,
                    childern: Vec::new(),
                })
            },
        }
    }
}

impl TaskControlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        self.inner.exclusive_access()
    }
}

pub struct TaskControlBlockInner {
    pub status: TaskStatus,
    pub cx: TaskContext,
    pub memory_set: MemorySet,
    pub exit_code: i32,
    pub parent: Option<Weak<TaskControlBlock>>,
    pub childern: Vec<Arc<TaskControlBlock>>,
}

/// task status: UnInit, Ready, Running, Exited
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    UnInit,
    Zombie,
}
