use {
    crate::mm::{
        address::PhysPageNum,
        page_table::{entry::PageTableEntry, frame::FrameTracker},
    },
    alloc::vec::Vec,
};

/// page table structure
struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}
