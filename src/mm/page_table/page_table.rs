use crate::mm::address::PhysPageNum;
use crate::mm::page_table::entry::PageTableEntry;
use crate::mm::page_table::frame::FrameTracker;
use alloc::vec::Vec;

/// page table structure
struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}
