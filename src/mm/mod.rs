pub mod buddy_system;
pub mod heap_allocator;
pub mod memory_set;
pub mod page_table;

pub fn init() {
    heap_allocator::init();
    page_table::init();
    memory_set::KERNEL_SPACE.exclusive_access().test();
}
