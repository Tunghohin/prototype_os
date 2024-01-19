pub mod address;
pub mod buddy_system;
pub mod heap_allocator;
pub mod linked_list;
pub mod page_table;

pub fn init() {
    heap_allocator::init();
}
