pub mod address;
pub mod heap_allocator;
pub mod page_table;
pub mod pages;

pub fn init() {
    heap_allocator::init();
}
