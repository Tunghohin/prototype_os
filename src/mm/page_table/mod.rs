pub mod entry;
pub mod frame;
pub mod page_table;

pub fn init() {
    frame::frame_allocator_init();
}
