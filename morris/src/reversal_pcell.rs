#![allow(dead_code)]
extern crate creusot_contracts;
use ::std::mem;
use ::std::rc::Rc;
use creusot_contracts::pcell::{PCell, PCellOwn};
use creusot_contracts::*;
enum List<T> {
    Empty,
    Cons(T, Rc<PCell<List<T>>>),
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::Empty
    }

    pub fn reverse_in_place(&mut self, mut seq: Ghost<Seq<PCellOwn<List<T>>>>) {
        let mut q = &mut List::Empty;
        let mut p = self;
        let mut index = 0;
        while let List::Cons(_, next) = p {
            unsafe {
                let perm = ghost!(seq.get_mut_ghost(*Int::new(index)).unwrap());
                let fwd = next.borrow_mut(perm);

                //j'aurais du faire directement
                // next.set(perm, *q), mais le borrow_checker renvoie une erreur.
                let old_q = mem::replace(q, List::Empty);
                let perm = ghost!(seq.get_mut_ghost(*Int::new(index)).unwrap());
                next.set(perm, old_q);

                //si j'interchange les deux lignes en bas, le borrowchecker ne passerais pas.
                p = fwd;
                q = p;
                index += 1;
            }
        }
    }
}
