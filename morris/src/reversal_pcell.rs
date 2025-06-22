#![allow(dead_code)]
extern crate creusot_contracts;
use ::std::rc::Rc;
use creusot_contracts::pcell::{PCell, PCellOwn};
use creusot_contracts::*;
pub struct Node<T> {
    elem: T,
    next: Rc<PCell<List<T>>>,
}

pub struct List<T> {
    head: Option<Node<T>>,
}
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn reverse_in_place(&mut self, mut seq: Ghost<Seq<PCellOwn<List<T>>>>) -> Self {
        let mut q = List { head: None };
        let mut p = self;
        let mut index = 0;
        let mut next; //I put here next to make it outlive the loop

        while !p.head.is_none() {
            let curr = p.head.take().unwrap();
            next = curr.next.clone();

            let perm = ghost!(seq.get_mut_ghost(Int::new(index).into_inner()).unwrap());
            unsafe {
                next.set(perm, q);
            }

            q = List { head: Some(curr) };

            let perm = ghost!(seq.get_mut_ghost(Int::new(index).into_inner()).unwrap());
            unsafe {
                p = next.borrow_mut(perm);
            }

            index += 1;
        }
        q
    }
}
