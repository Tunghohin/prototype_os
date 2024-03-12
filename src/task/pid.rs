use crate::misc::bitmanip::low_bit;
use crate::println;
use crate::sync::upsafecell::UPSafeCell;
use crate::sysconfig::{PAGE_SIZE, TRAMPOLINE, USER_STACK_SIZE};
use alloc::vec::Vec;
use core::borrow::BorrowMut;
use core::fmt::Debug;
use lazy_static::*;

/// bitmap for pid allocation, range from 0 to 1023
struct PidBitMap {
    inner: [usize; 16],
}

impl PidBitMap {
    /// Creates a new [`PidBitMap`].
    fn new() -> Self {
        PidBitMap {
            inner: [usize::MAX; 16],
        }
    }

    /// Returns the request of this [`PidBitMap`].
    fn request(&mut self) -> Option<usize> {
        for i in 0..16 {
            if self.inner[i] == 0 {
                // means all bit are allocated on this field
                continue;
            }
            let lowbit: usize = low_bit(self.inner[i]);
            self.inner[i] &= !lowbit;

            return Some(i * 64 + lowbit.trailing_zeros() as usize);
        }
        None
    }

    fn release(&mut self, pid: usize) {
        assert!(pid < 1204, "range: 0 - 1023");
        self.inner[pid / 64] |= 1 << (pid % 64);
    }
}

/// Pid allocator
struct BitmapAllocator {
    bitmap: PidBitMap,
}

impl BitmapAllocator {
    pub fn new() -> Self {
        BitmapAllocator {
            bitmap: PidBitMap::new(),
        }
    }

    fn request(&mut self) -> Option<PidHandle> {
        match self.bitmap.request() {
            Some(x) => Some(PidHandle(x)),
            None => None,
        }
    }

    fn release(&mut self, handle: &PidHandle) {
        self.bitmap.release(handle.0);
    }
}

impl Debug for BitmapAllocator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PidAllocator\n")
            .field("field  0", &format_args!("{:b}\n", self.bitmap.inner[0]))
            .field("field  1", &format_args!("{:b}\n", self.bitmap.inner[1]))
            .field("field  2", &format_args!("{:b}\n", self.bitmap.inner[2]))
            .field("field  3", &format_args!("{:b}\n", self.bitmap.inner[3]))
            .field("field  4", &format_args!("{:b}\n", self.bitmap.inner[4]))
            .field("field  5", &format_args!("{:b}\n", self.bitmap.inner[5]))
            .field("field  6", &format_args!("{:b}\n", self.bitmap.inner[6]))
            .field("field  7", &format_args!("{:b}\n", self.bitmap.inner[7]))
            .field("field  8", &format_args!("{:b}\n", self.bitmap.inner[8]))
            .field("field  9", &format_args!("{:b}\n", self.bitmap.inner[9]))
            .field("field 10", &format_args!("{:b}\n", self.bitmap.inner[10]))
            .field("field 11", &format_args!("{:b}\n", self.bitmap.inner[11]))
            .field("field 12", &format_args!("{:b}\n", self.bitmap.inner[12]))
            .field("field 13", &format_args!("{:b}\n", self.bitmap.inner[13]))
            .field("field 14", &format_args!("{:b}\n", self.bitmap.inner[14]))
            .field("field 15", &format_args!("{:b}\n", self.bitmap.inner[15]))
            .finish()
    }
}

lazy_static! {
    static ref PID_ALLOCATOR: UPSafeCell<BitmapAllocator> =
        unsafe { UPSafeCell::new(BitmapAllocator::new()) };
    static ref KSTACK_ALLOCATOR: UPSafeCell<BitmapAllocator> =
        unsafe { UPSafeCell::new(BitmapAllocator::new()) };
}

pub fn pid_alloc() -> PidHandle {
    PID_ALLOCATOR.exclusive_access().request().unwrap()
}

pub struct KernelStack(usize);

pub fn kstack_alloc() -> (KernelStack, usize, usize) {
    let kstack_id = KSTACK_ALLOCATOR.exclusive_access().request().unwrap().0;
    let kstack_top = TRAMPOLINE - kstack_id * (USER_STACK_SIZE + PAGE_SIZE);
    let kstack_bottom = kstack_top - USER_STACK_SIZE;
    (KernelStack(kstack_id), kstack_bottom, kstack_top)
}

/// Abstraction of Process Identifier
#[derive(Debug)]
pub struct PidHandle(usize);

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.exclusive_access().release(self);
    }
}

/// Use it after comment out drop trait of PidHandle
#[allow(unused)]
pub fn pid_alloc_test() {
    // let mut holder: Vec<PidHandle> = Vec::new();
    for i in 0..127 {
        PID_ALLOCATOR.exclusive_access().request();
    }
    PID_ALLOCATOR
        .exclusive_access()
        .release(PidHandle(1).borrow_mut());
    PID_ALLOCATOR
        .exclusive_access()
        .release(PidHandle(5).borrow_mut());
    println!("{:?}", PID_ALLOCATOR.exclusive_access());

    PID_ALLOCATOR.exclusive_access().request();
    println!("{:?}", PID_ALLOCATOR.exclusive_access());

    PID_ALLOCATOR
        .exclusive_access()
        .release(PidHandle(110).borrow_mut());
    PID_ALLOCATOR
        .exclusive_access()
        .release(PidHandle(113).borrow_mut());
    PID_ALLOCATOR.exclusive_access().request();
    println!("{:?}", PID_ALLOCATOR.exclusive_access());

    PID_ALLOCATOR.exclusive_access().request();
    println!("{:?}", PID_ALLOCATOR.exclusive_access());
}
