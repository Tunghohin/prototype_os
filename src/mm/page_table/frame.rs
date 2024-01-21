use crate::mm::address::PhysPageNum;
use crate::sysconfig::MEMORY_END;
use alloc::vec::Vec;

#[derive(Debug)]
/// tracking the allocation and deallocation of a page frame
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

trait FrameAllocator {
    fn new(start_ppn: PhysPageNum, end_ppn: PhysPageNum) -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}
type FrameAllocatorImpl = StackFrameAllocator;

pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl FrameAllocator for StackFrameAllocator {
    fn new(start_ppn: PhysPageNum, end_ppn: PhysPageNum) -> Self {
        StackFrameAllocator {
            current: start_ppn.0,
            end: end_ppn.0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(PhysPageNum(ppn))
        } else if self.current == self.end {
            None
        } else {
            self.current += 1;
            Some(PhysPageNum(self.current - 1))
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
}
