pub mod address;
pub mod heap_allocator;
pub mod pages;
pub mod pages_table;

pub fn init() {
    heap_allocator::init();
}
