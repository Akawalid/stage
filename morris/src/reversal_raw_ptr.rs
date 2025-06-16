#![allow(dead_code)]
extern crate creusot_contracts;
use ::std::ptr;
use creusot_contracts::ptr_own::{PtrOwn, RawPtr};
use creusot_contracts::*;
pub struct Node<T> {
    elem: T,
    next: RawPtr<Node<T>>,
}

pub struct List<T> {
    head: RawPtr<Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: ptr::null() }
    }
    pub fn reverse_in_place(&mut self, mut seq: Ghost<Seq<PtrOwn<Node<T>>>>) {
        let mut q = ptr::null();
        let mut p = self.head;
        let mut index = 0;
        while !p.is_null() {
            let next = unsafe {
                let p2 = PtrOwn::as_mut(
                    p,
                    ghost!(seq.get_mut_ghost(Int::new(index).into_inner()).unwrap()),
                );
                let next = (*p2).next;

                (*p2).next = q;

                next
            };
            q = p;
            p = next;
            index = index + 1;
        }
        self.head = q;
    }
}

// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         let mut p = self.head;
//         while !p.is_null() {
//             unsafe {
//                 let next = (*p).next;
//                 drop(p);
//                 p = next;
//             }
//         }
//     }
// }
