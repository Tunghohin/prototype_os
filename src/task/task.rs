use crate::sync::upsafecell::UPSafeCell;
use crate::task::pid::PidHandle;

pub struct TaskContrlBlock {
    pub pid: PidHandle,
    inner: UPSafeCell<TaskContrlBlockInner>,
}

struct TaskContrlBlockInner {}
