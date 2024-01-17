use {crate::mm::address::PhyPageNum, crate::sysconfig::MEMORY_END};

#[derive(Debug)]
/// tracking the allocation and deallocation of a page frame
pub struct FrameTracker {
    pub ppn: PhyPageNum,
}
