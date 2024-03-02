use crate::hal::*;
use crate::mm::page_table::frame::{frame_alloc, FrameTracker};
use alloc::vec;
use alloc::vec::Vec;

/// page table structure
pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    /// Create a PageTable and alloc a frame, pointing root_ppn to the PhysPageNum of the frame
    pub fn new() -> Self {
        let root_frame = frame_alloc().unwrap();
        PageTable {
            root_ppn: root_frame.ppn,
            frames: vec![root_frame],
        }
    }

    /// Find PageTableEntry by VirtPageNum. if does not exist, create a 4KB frame with 512 PageTableEntry in it
    fn find_pte_or_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let mut ppn = self.root_ppn;
        for level in 1..=Arch::LEVEL {
            let index = vpn.get_pte_index(level);
            let entry = &mut ppn.get_pte_array_mut()[index];
            if level == Arch::LEVEL {
                return Some(entry);
            }
            if !entry.is_valid() {
                let frame = frame_alloc().unwrap();
                *entry = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = entry.get_ppn();
        }

        // not supposed to get here
        None
    }

    /// Find PageTableEntry by VirtPageNum. if does not exist, retuen None
    fn find_pte(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let mut ppn = self.root_ppn;
        for level in 1..=Arch::LEVEL {
            let index = vpn.get_pte_index(level);
            let entry = &mut ppn.get_pte_array_mut()[index];
            if level == Arch::LEVEL {
                return Some(entry);
            }
            if !entry.is_valid() {
                return None;
            }
            ppn = entry.get_ppn();
        }
        None
    }

    /// Map a VirtPageNum to a PhysPageNum
    fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_or_create(vpn).expect("Map failed!");
        assert!(!pte.is_valid(), "{:?} is mapped before!", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    /// Unmap a VirtPageNum
    fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self
            .find_pte(vpn)
            .expect("Unmap failed! Page table entry not found.");
        *pte = PageTableEntry::new(0.into(), PTEFlags::from_bits(0).unwrap());
    }
}
