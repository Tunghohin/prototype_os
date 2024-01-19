//! Provide the intrusive LinkedList
#![allow(dead_code)]

use core::ptr;

#[derive(Copy, Clone)]
pub struct LinkedList {
    head: *mut usize,
}

unsafe impl Send for LinkedList {}

impl LinkedList {
    pub const fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn push(&mut self, item: *mut usize) {
        unsafe {
            *item = self.head as usize;
        }
        self.head = item;
    }

    pub fn pop(&mut self) -> Option<*mut usize> {
        match self.head.is_null() {
            true => None,
            false => {
                let item = self.head;
                self.head = unsafe { *item as *mut usize };
                Some(item)
            }
        }
    }
}
