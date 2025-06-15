#![allow(dead_code)]

use std::ptr;

struct Node<T> {
    elem: T,
    next: *mut Node<T>,
}

pub struct List<T> {
    head: *mut Node<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
        }
    }

    pub fn reverse_in_place(&mut self) {
        let mut q = ptr::null_mut();
        let mut p = self.head;

        while !p.is_null() {
            let next = unsafe {
                let next = (*p).next;
                (*p).next = q;
                next
            };
            q = p;
            p = next;
        }
        self.head = q;
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut p = self.head;
        while !p.is_null() {
            unsafe {
                let next = (*p).next;
                drop(Box::from_raw(p));
                p = next;
            }
        }
    }
}
