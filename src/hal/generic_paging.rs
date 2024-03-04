use crate::hal::PhysPageNum;

pub trait PagingMetaData {
    const LEVEL: usize;
}

pub trait GenericPTEFlag {}

pub trait GenericPagetableEntry<F: GenericPTEFlag> {
    /// create a new pagetable entry
    fn new(ppn: PhysPageNum, flags: F) -> Self;
    /// validity
    fn is_valid(&self) -> bool;
    /// readable
    fn is_readable(&self) -> bool;
    /// writable
    fn is_writable(&self) -> bool;
    /// executable
    fn is_executable(&self) -> bool;
    /// u-mode accessibla
    fn is_uaccessible(&self) -> bool;
    /// global mapping
    fn is_gmapping(&self) -> bool;
    /// accessed
    fn is_accessed(&self) -> bool;
    /// dirty
    fn is_dirty(&self) -> bool;
    /// Get PhysPageNum
    fn get_ppn(&self) -> PhysPageNum;
}
