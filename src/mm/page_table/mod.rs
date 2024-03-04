pub mod frame;
pub mod page_table;

pub use page_table::PageTable;

pub fn init() {
    frame::frame_allocator_init();
}
