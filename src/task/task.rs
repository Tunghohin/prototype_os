#![allow(dead_code)]

use crate::loader::get_app_data_by_name;
use crate::mm::memory_set::{MapSegment, MapType, MemorySet, KERNEL_SPACE};
use crate::sync::upsafecell::UPSafeCell;
use crate::sysconfig::TRAP_CONTEXT_BASE;
use crate::task::pid::{kstack_alloc, pid_alloc};
use crate::task::pid::{KernelStack, PidHandle};
use crate::{hal::*, print, println};
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use core::cell::RefMut;
use lazy_static::*;

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

        let cx = TaskContext::goto_trap_return(kstack.get_kstack_top());
        let trap_cx_ppn = memory_set.translate_ppn(VirtAddr::from(TRAP_CONTEXT_BASE).into());
        let trap_cx: &mut TrapContext = PhysAddr::from(trap_cx_ppn).get_mut();
        *trap_cx = TrapContext::task_init_cx(entry_point, user_stack_top, kstack.get_kstack_top());

        TaskControlBlock {
            pid,
            kstack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    trap_cx_ppn,
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
    /// Trap context of this task
    pub trap_cx_ppn: PhysPageNum,
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

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> =
        Arc::new(TaskControlBlock::new(get_app_data_by_name("initproc")));
}
