use crate::task::task::TaskContrlBlock;
use alloc::sync::Arc;

pub struct CPU {
    current: Option<Arc<TaskContrlBlock>>,
}
