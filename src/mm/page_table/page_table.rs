use {super::entry, crate::mm::address::PhyPageNum};

/// page table structure
struct PageTable {
    root_ppn: PhyPageNum,
}
