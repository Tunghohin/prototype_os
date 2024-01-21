#![allow(dead_code)]

use crate::mm::linked_list;
use core::alloc::{GlobalAlloc, Layout};
use core::cmp::{max, min};
use core::mem::size_of;
use core::ops::Deref;
use core::ptr::NonNull;
use spin::Mutex;

const MAX_ORDER: usize = 64;

pub struct Heap {
    // buddy system with max order of 32
    free_list: [linked_list::LinkedList; MAX_ORDER],

    // statistics
    user: usize,
    allocated: usize,
    total: usize,
}

impl Heap {
    const fn new() -> Self {
        Heap {
            free_list: [linked_list::LinkedList::new(); MAX_ORDER],
            user: 0,
            allocated: 0,
            total: 0,
        }
    }

    fn empty() -> Self {
        Self::new()
    }

    pub unsafe fn init(&mut self, mut start: usize, mut end: usize) {
        // avoid unaligned access on some platforms
        start = (start + size_of::<usize>() - 1) & (!size_of::<usize>() + 1);
        end = end & (!size_of::<usize>() + 1);
        assert!(start <= end);

        let mut current_start = start;
        while current_start + size_of::<usize>() <= end {
            let low_bit = current_start & (!current_start + 1);
            let size = min(low_bit, prev_power_of_two(end - current_start));
            self.total += size;
            self.free_list[size.trailing_zeros() as usize].push(current_start as *mut usize);
            current_start += size;
        }
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );

        let alloc_type = size.trailing_zeros() as usize;
        let split_pos = match (alloc_type..MAX_ORDER).find(|&i| !self.free_list[i].is_empty()) {
            Some(split_pos) => split_pos,
            None => {
                return Err(());
            }
        };

        (alloc_type..split_pos).rev().for_each(|i| {
            let seg = self.free_list[i]
                .pop()
                .expect("current block should have free space now");
            self.free_list[i - 1].push(seg);
            self.free_list[i - 1].push((seg as usize + (1 << (i - 1))) as *mut usize);
        });

        if let Some(result) = NonNull::new(
            self.free_list[alloc_type]
                .pop()
                .expect("current block should have free space now") as *mut u8,
        ) {
            self.user += layout.size();
            self.allocated += size;
            Ok(result)
        } else {
            Err(())
        }
    }

    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        let alloc_type = size.trailing_zeros() as usize;

        self.free_list[alloc_type].push(ptr.as_ptr() as *mut usize);

        let mut current_ptr = ptr.as_ptr() as usize;
        let mut current_type = alloc_type;
        while current_type < MAX_ORDER {
            let buddy = current_ptr ^ (1 << current_type);
            if let Some(seg) = self.free_list[current_type]
                .iter_mut()
                .find(|seg| seg.value() as usize == buddy)
            {
                seg.pop();
                self.free_list[alloc_type].pop();
                current_ptr = min(current_ptr, buddy);
                current_type += 1;
                self.free_list[alloc_type].push(current_ptr as *mut usize);
            } else {
                break;
            }
        }
    }
}

pub struct LockedHeap(Mutex<Heap>);

impl LockedHeap {
    /// Creates an empty heap
    pub const fn new() -> LockedHeap {
        LockedHeap(Mutex::new(Heap::new()))
    }

    /// Creates an empty heap
    pub const fn empty() -> LockedHeap {
        LockedHeap(Mutex::new(Heap::new()))
    }
}

impl Deref for LockedHeap {
    type Target = Mutex<Heap>;

    fn deref(&self) -> &Mutex<Heap> {
        &self.0
    }
}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(NonNull::new_unchecked(ptr), layout)
    }
}

/// return the largest power of two less or equal to num
/// just do 1 shift left sizeof usize minus leading zeros of num
/// #Example
/// basic_usage:
/// ```
/// assert_eq!(prev_power_of_two(0b00100100), 0b100000);
/// ```
pub(crate) fn prev_power_of_two(num: usize) -> usize {
    1 << (8 * (size_of::<usize>()) - num.leading_zeros() as usize - 1)
}
