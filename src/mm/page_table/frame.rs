use crate::hal::{PhysAddr, PhysPageNum};
use crate::sync::upsafecell::UPSafeCell;
use crate::sysconfig::MEMORY_END;
use alloc::vec::Vec;
use lazy_static::*;

lazy_static! {
    pub static ref GLOBAL_FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> =
        unsafe { UPSafeCell::new(FrameAllocatorImpl::new()) };
}
trait FrameAllocator {
    fn new() -> Self;
    fn init(&mut self, start_ppn: PhysPageNum, end_ppn: PhysPageNum);
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}
type FrameAllocatorImpl = StackFrameAllocator;

#[derive(Debug)]
/// tracking the allocation and deallocation of a page frame
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    fn new(ppn: PhysPageNum) -> Self {
        let bytes_array = ppn.get_bytes_array();
        bytes_array.fill(0);
        FrameTracker { ppn }
    }
}

pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        StackFrameAllocator {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else if self.current == self.end {
            None
        } else {
            self.current += 1;
            Some((self.current - 1).into())
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        if ppn.0 >= self.current
            || self
                .recycled
                .iter()
                .any(|&recycled_ppn| recycled_ppn == ppn.0)
        {
            panic!("Frame ppn={:#x} has not been allocated!", ppn.0);
        } else {
            self.recycled.push(ppn.0);
        }
    }

    fn init(&mut self, start_ppn: PhysPageNum, end_ppn: PhysPageNum) {
        self.current = start_ppn.0;
        self.end = end_ppn.0;
    }
}

pub fn frame_allocator_init() {
    extern "C" {
        fn ekernel();
    }
    let start_pa: PhysAddr = (ekernel as usize).into();
    let end_pa: PhysAddr = MEMORY_END.into();
    GLOBAL_FRAME_ALLOCATOR
        .exclusive_access()
        .init(start_pa.ceil(), end_pa.floor());
}

pub fn frame_alloc() -> Option<FrameTracker> {
    GLOBAL_FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(FrameTracker::new)
}

pub fn frame_dealloc(ppn: PhysPageNum) {
    GLOBAL_FRAME_ALLOCATOR.exclusive_access().dealloc(ppn)
}
