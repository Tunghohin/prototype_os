use crate::mm::address::{PhysPageNum, VirtPageNum};
use crate::mm::page_table::entry::{PageTableEntry, PTEIDX_MASK_SV39, PTEIDX_OFFSET_SV39};
use crate::mm::page_table::frame::{frame_alloc, FrameTracker};
use alloc::vec;
use alloc::vec::Vec;

use super::entry::PTEFlags;

/// page table structure
struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    /// Create a PageTable and alloc a frame, pointing root_ppn to the PhysPageNum of the frame
    fn new() -> Self {
        let root_frame = frame_alloc().unwrap();
        PageTable {
            root_ppn: root_frame.ppn,
            frames: vec![root_frame],
        }
    }

    /// Find PageTableEntry by VirtPageNum. if does not exist, create a 4KB frame with 512 PageTableEntry in it
    fn find_pte_or_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let mut ppn = self.root_ppn;
        for level in 1..=3 {
            let index = ppn.0 & (PTEIDX_MASK_SV39 << (level * PTEIDX_OFFSET_SV39));
            let entry = &mut ppn.get_pte_array()[index];
            if level == 3 {
                return Some(entry);
            }
            if !entry.is_valid() {
                let frame = frame_alloc().unwrap();
                *entry = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = entry.get_ppn();
        }
        None
    }

    /// Find PageTableEntry by VirtPageNum. if does not exist, retuen None
    fn find_pte(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let mut ppn = self.root_ppn;
        for level in 1..=3 {
            let index = ppn.0 & (PTEIDX_MASK_SV39 << (level * PTEIDX_OFFSET_SV39));
            let entry = &mut ppn.get_pte_array()[index];
            if level == 3 {
                return Some(entry);
            }
            if !entry.is_valid() {
                return None;
            }
            ppn = entry.get_ppn();
        }
        None
    }
}
