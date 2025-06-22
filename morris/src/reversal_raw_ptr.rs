#![allow(dead_code)]
extern crate creusot_contracts;
use ::std::ptr;
use creusot_contracts::ptr_own::{PtrOwn, RawPtr};
//use creusot_contracts::std::ptr::PointerExt;
use creusot_contracts::*;
pub struct Node<T> {
    elem: T,
    next: RawPtr<Node<T>>,
}

impl<T: PartialEq> Node<T> {
    #[predicate]
    fn list_aux(l: RawPtr<Self>, perm_seq: &mut Seq<PtrOwn<Node<T>>>, i: Int) -> bool {
        //On n'aura pas vraiment besoin de l puisque on suppose que perm_seq ne peut pas etre autre chose que la liste
        //des permissions de l
        pearlite! {
            if l.is_null_logic() {
                perm_seq.len() == i
            } else {
                 if i < perm_seq.len(){
                    let ptr = perm_seq[i].ptr();
                    l == ptr && Self::list_aux(perm_seq[i].val().next, perm_seq, i + 1)
                } else {
                    false
                }
            }
        }
    }

    #[predicate]
    fn list(l: RawPtr<Self>, perm_seq: &mut Seq<PtrOwn<Node<T>>>) -> bool {
        //On n'aura pas vraiment besoin de l puisque on suppose que perm_seq ne peut pas etre autre chose que la liste
        //des permissions de l
        pearlite! {
            Self::list_aux(l, perm_seq, 0)
        }
    }

    pub fn empty() -> (RawPtr<Self>, Ghost<Seq<PtrOwn<Node<T>>>>) {
        (ptr::null(), Seq::new())
    }

    pub fn cons(e: T, l: RawPtr<Self>, seq: &mut Ghost<Seq<PtrOwn<Node<T>>>>) -> RawPtr<Self> {
        let (raw, own) = PtrOwn::new(Node { elem: e, next: l });
        ghost!(seq.push_front_ghost(own.into_inner()));
        raw
    }
    pub fn nth(mut p: RawPtr<Self>, nth: i128, seq: &Ghost<Seq<PtrOwn<Node<T>>>>) -> &T {
        //requires nth >= 0
        let mut i = 0;

        loop {
            let rw = unsafe {
                PtrOwn::as_ref(p, ghost!(seq.get_ghost(Int::new(i).into_inner()).unwrap()))
            };

            if i == nth {
                return &rw.elem;
            }

            p = rw.next;
            i += 1;
        }
    }

    #[requires(Self::list(p, &mut**seq))]
    #[ensures(Self::list(result, &mut seq.reverse()))]
    pub fn reverse_in_place(
        mut p: RawPtr<Self>,
        seq: &mut Ghost<Seq<PtrOwn<Node<T>>>>,
    ) -> RawPtr<Self> {
        //requires p n'est pas un lasso
        let mut q = ptr::null();
        let mut index = ghost!(0int);
        #[invariant(exists<a1: Seq<PtrOwn<Node<T>>>, a2:Seq<PtrOwn<Node<T>>>> 
            Self::list(q, &mut a1) 
            && Self::list (p, &mut a2) 
            && seq.reverse() == a2.reverse().concat(a1))]
        while !p.is_null() {
            let p2 = unsafe { PtrOwn::as_mut(p, ghost!(seq.get_mut_ghost(*index).unwrap())) };
            let next = p2.next;
            p2.next = q;
            q = p;
            p = next;
            ghost!(*index = *index + 1int);
        }
        q
    }
}

pub fn list_of_vector1<T: PartialEq>(
    mut vec: Vec<T>,
) -> (RawPtr<Node<T>>, Ghost<Seq<PtrOwn<Node<T>>>>) {
    //Takes possession of elements in the vector
    let (mut l, mut seq) = Node::empty();

    while let Some(v) = vec.pop() {
        l = Node::cons(v, l, &mut seq);
    }
    (l, seq)
}

pub fn list_of_vector2<T: Clone + PartialEq>(
    vec: &Vec<T>,
) -> (RawPtr<Node<T>>, Ghost<Seq<PtrOwn<Node<T>>>>) {
    let (mut l, mut seq) = Node::empty();

    let mut i = 1;
    while i <= vec.len() {
        l = Node::cons(vec[vec.len() - i].clone(), l, &mut seq);
        i += 1;
    }
    (l, seq)
}

#[trusted]
pub fn print(_: &str) {}

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
