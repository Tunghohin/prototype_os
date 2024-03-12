#![allow(dead_code)]

use crate::hal::*;
use crate::mm::memory_set::{MapSegment, MapType, MemorySet, KERNEL_SPACE};
use crate::sync::upsafecell::UPSafeCell;
use crate::task::pid::{kstack_alloc, pid_alloc};
use crate::task::pid::{KernelStack, PidHandle};
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;

pub struct TaskContrlBlock {
    pub pid: PidHandle,
    pub kstack: KernelStack,
    inner: UPSafeCell<TaskContrlBlockInner>,
}

impl TaskContrlBlock {
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
        TaskContrlBlock {
            pid,
            kstack,
            inner: unsafe { core::mem::zeroed::<UPSafeCell<TaskContrlBlockInner>>() },
        }
    }
}

struct TaskContrlBlockInner {
    status: TaskStatus,
    cx: TaskContext,
    memset: MemorySet,
    exit_code: i32,
    parent: Option<Weak<TaskContrlBlock>>,
    childern: Vec<Arc<TaskContrlBlock>>,
}

/// task status: UnInit, Ready, Running, Exited
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    UnInit,
    Zombie,
}
