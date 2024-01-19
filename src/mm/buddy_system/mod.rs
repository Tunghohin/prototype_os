use {
    crate::mm::linked_list,
    core::{cmp::min, mem::size_of},
};

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
    fn new() -> Self {
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
}

/// return the largest power of two less or equal to num
/// such as 0b00100100 -> 0b100000
/// just do 1 shift left sizeof usize minus leading zeros of num
pub(crate) fn prev_power_of_two(num: usize) -> usize {
    1 << (8 * (size_of::<usize>()) - num.leading_zeros() as usize - 1)
}
