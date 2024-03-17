#![allow(dead_code)]

use crate::hal::*;
use crate::mm::memory_set::{MapSegment, MapType, MemorySet, KERNEL_SPACE};
use crate::sync::upsafecell::UPSafeCell;
use crate::task::pid::{kstack_alloc, pid_alloc};
use crate::task::pid::{KernelStack, PidHandle};
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use core::cell::RefMut;

pub struct TaskControlBlock {
    /// Process id handle of this task
    pub pid: PidHandle,
    /// Kernel stack address wrapper
    pub kstack: KernelStack,
    /// Task control block inner with exclusive access control
    inner: UPSafeCell<TaskControlBlockInner>,
}

impl TaskControlBlock {
    /// New task which is ready to run
    pub fn new(elf_data: &[u8]) -> Self {
        let (memory_set, user_stack_top, entry_point) = MemorySet::new_task(elf_data);
        let pid = pid_alloc();
        let kstack = kstack_alloc(&pid);
        KERNEL_SPACE.exclusive_access().insert_segment(
            MapSegment::new(
                kstack.get_kstack_bottom().into(),
                kstack.get_kstack_top().into(),
                MapType::Framed,
                MapPermission::W | MapPermission::R,
            ),
            None,
        );
        let cx = TaskContext::goto_trap_return(kstack.get_kstack_top());
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

    pub fn get_pid(&self) -> usize {
        self.pid.0
    }

    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        self.inner.exclusive_access()
    }
}

pub struct TaskControlBlockInner {
    /// Context of this task
    pub cx: TaskContext,
    /// Status of this task
    pub status: TaskStatus,
    /// Memory set of this task
    pub memory_set: MemorySet,
    /// Exit status of this task
    pub exit_code: i32,
    /// parent task of this task
    pub parent: Option<Weak<TaskControlBlock>>,
    /// childern of this task
    pub childern: Vec<Arc<TaskControlBlock>>,
}

/// task status: UnInit, Ready, Running, Exited
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// Ready to run
    Ready,
    /// Running
    Running,
    /// Uninitialized
    UnInit,
    /// Zombie
    Zombie,
}
