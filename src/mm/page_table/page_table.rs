#![allow(dead_code)]

use crate::hal::*;
use crate::misc::range::StepByOne;
use crate::mm::page_table::frame::{frame_alloc, FrameTracker};
use crate::task::cpu::current_task;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp;

/// page table structure
pub struct PageTable {
    pub root_ppn: PhysPageNum,
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
        for level in (0..Arch::LEVEL).rev() {
            let index = vpn.get_pte_index(level);
            let entry = &mut ppn.get_pte_array_mut()[index];
            if level == 0 {
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
    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let mut ppn = self.root_ppn;
        for level in (0..Arch::LEVEL).rev() {
            let index = vpn.get_pte_index(level);
            let entry = &mut ppn.get_pte_array_mut()[index];
            if level == 0 {
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
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_or_create(vpn).expect("Map failed!");
        assert!(!pte.is_valid(), "{:?} is mapped before!", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    /// Unmap a VirtPageNum
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self
            .find_pte(vpn)
            .expect("Unmap failed! Page table entry not found.");
        *pte = PageTableEntry::new(0.into(), PTEFlags::from_bits(0).unwrap());
    }

    pub fn translate_pte(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| *pte)
    }

    pub fn translate_ppn(&self, vpn: VirtPageNum) -> PhysPageNum {
        self.translate_pte(vpn)
            .expect("Not a valid VirtPageNum")
            .get_ppn()
    }

    pub fn get_root_ppn(&self) -> PhysPageNum {
        self.root_ppn
    }
}
