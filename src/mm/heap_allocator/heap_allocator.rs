use crate::{mm::buddy_system::LockedHeap, sysconfig::KERNEL_HEAP_SIZE};

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

static mut KERNEL_HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init() {
    unsafe {
        let start = KERNEL_HEAP.as_ptr() as usize;
        HEAP_ALLOCATOR.lock().init(start, start + KERNEL_HEAP_SIZE);
    }
}
