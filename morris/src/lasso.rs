// #![allow(dead_code)]
// extern crate creusot_contracts;
// use ::std::ptr;
// use creusot_contracts::ptr_own::{PtrOwn, RawPtr};
// use creusot_contracts::*;
// pub struct Node<T> {
//     elem: T,
//     pub next: RawPtr<Node<T>>,
// }

// impl<T> Node<T> {
//     #[predicate]
//     #[variant(perm_seq.len())]
//     fn list_segment(l: RawPtr<Self>, q: RawPtr<Self>, perm_seq: Seq<PtrOwn<Node<T>>>) -> bool {
//         pearlite! {
//             if l.is_null_logic() || l == q {
//                 perm_seq.len() == 0
//             }
//             else {
//                  if perm_seq.len() > 0 {
//                     let ptr = perm_seq[0].ptr();
//                     l == ptr && Self::list_segment(perm_seq[0].val().next, q, perm_seq.tail())
//                 } else {
//                     false
//                 }
//             }
//         }
//     }

//     #[predicate]
//     fn lasso(l: RawPtr<Self>, perm_seq: Seq<PtrOwn<Node<T>>>) -> bool {
//         pearlite! {
//             exists<j: RawPtr<Self>, n: Int>
//             Self::list_segment(l, j, perm_seq.subsequence(0, n))
//             && Self::list_segment(j, j, perm_seq.subsequence(n, perm_seq.len()))
//         }
//     }

//     // #[ensures(Self::list(result.0, *result.1))]
//     // #[ensures(result.0.is_null_logic())]
//     // pub fn empty() -> (RawPtr<Self>, Ghost<Seq<PtrOwn<Node<T>>>>) {
//     //     (ptr::null(), Seq::new())
//     // }

//     // #[requires(Self::list(l, **seq))]
//     // #[ensures(Self::list(result,  *^seq))]
//     // #[ensures(forall<i:Int> 0 <= i && i < (^seq).tail().len() ==> seq[i] == (^seq).tail()[i])]
//     // #[ensures((^seq)[0].val().elem == e)]
//     // #[ensures((^seq)[0].ptr() == result)]
//     // #[ensures((^seq).len() == seq.len() + 1)]
//     // pub fn cons(e: T, l: RawPtr<Self>, seq: &mut Ghost<Seq<PtrOwn<Node<T>>>>) -> RawPtr<Self> {
//     //     let (raw, own) = PtrOwn::new(Node { elem: e, next: l });

//     //     let _seq2 = snapshot!(**seq);
//     //     ghost!(seq.push_front_ghost(own.into_inner()));
//     //     proof_assert!(*_seq2 == seq.tail());

//     //     raw
//     // }

//     // #[requires(Self::list(p, **seq))]
//     // #[requires(0 <= nth@ && nth@ < seq.len() )]
//     // #[ensures(seq[nth@].val().elem == *result)]
//     // pub fn nth(mut p: RawPtr<Self>, nth: i128, seq: &Ghost<Seq<PtrOwn<Node<T>>>>) -> &T {
//     //     //requires nth >= 0
//     //     let mut i = 0;
//     //     proof_assert!(**seq == seq.subsequence(0, seq.len()));
//     //     #[invariant(0 <= i@ && i@ <= nth@)]
//     //     #[invariant(Self::list(p, seq.subsequence(i@, seq.len())))]
//     //     loop {
//     //         let rw = unsafe {
//     //             PtrOwn::as_ref(p, ghost!(seq.get_ghost(Int::new(i).into_inner()).unwrap()))
//     //         };

//     //         if i == nth {
//     //             return &rw.elem;
//     //         }

//     //         p = rw.next;
//     //         proof_assert!(seq.subsequence(i@, seq.len()).tail() == seq.subsequence(i@+1, seq.len()));
//     //         i += 1;
//     //     }
//     // }

//     #[predicate]
//     pub fn inverse(seq: Seq<PtrOwn<Node<T>>>, other: Seq<PtrOwn<Node<T>>>, lb: Int, lh: Int) -> bool
//     where
//         T: Sized,
//     {
//         pearlite! {
//              forall<i: Int>
//             lb <= i && i < lh
//             ==> seq[i].val().elem == other[other.len() - i - 1].val().elem
//         }
//     }

//     #[logic]
//     #[requires(Self::lasso(l, perm_seq))]
//     #[ensures(perm_seq[result.1].ptr() == result.0)]
//     #[ensures(Self::list_segment(l, result.0, perm_seq.subsequence(0, result.1)))]
//     #[ensures(Self::list_segment(l, l, perm_seq.subsequence(result.1, perm_seq.len())))]
//     pub fn lasso_node(l: RawPtr<Self>, perm_seq: Seq<PtrOwn<Node<T>>>) -> (RawPtr<Self>, Int) {
//         //We need to retreive the head of the cycle, coloriage....plus rapide
//     }

//     #[requires(Self::lasso(p, **seq))]
//     #[ensures(Self::lasso(result, *^seq))]
//     #[ensures(seq.len() == (^seq).len())]
//     #[ensures(Self::inverse(**seq, *^seq, 0, (*^seq).len()))]
//     #[ensures(seq.subsequence(0, lasso_node(p, seq).1) == )]
//     pub fn reverse_in_place(
//         mut p: RawPtr<Self>,
//         seq: &mut Ghost<Seq<PtrOwn<Node<T>>>>,
//     ) -> RawPtr<Self> {
//         //requires p n'est pas un lasso

//         snapshot! {
//             let _ = Seq::<T>::ext_eq;
//         };
//         let mut q: *const Node<T> = ptr::null();
//         let mut reverted_seq = Seq::new();
//         let _seq0 = snapshot!(**seq);

//         // #[invariant(Self::list(q, *reverted_seq))]
//         // #[invariant(Self::list(p, **seq))]
//         // #[invariant(Self::inverse(_seq0.subsequence(0, reverted_seq.len()), *reverted_seq, 0, reverted_seq.len()))]
//         // #[invariant(reverted_seq.len() + seq.len() == _seq0.len())]
//         // #[invariant(**seq == _seq0.subsequence(reverted_seq.len(), _seq0.len()))]
//         // #[invariant(inv(seq))]
//         // #[invariant(inv(reverted_seq))]
//         while !p.is_null() {
//             let _sloop_entry = snapshot!(**seq);
//             let _revs_loop_entry = snapshot!(*reverted_seq);
//             let p2 =
//                 unsafe { PtrOwn::as_mut(p, ghost!(seq.get_mut_ghost(*ghost!(0int)).unwrap())) };
//             let next = p2.next;
//             p2.next = q;
//             q = p;
//             p = next;
//             let _sloop_exit = snapshot!(**seq);

//             ghost!((*reverted_seq).push_front_ghost(seq.pop_front_ghost().unwrap()));
//             // proof_assert!(reverted_seq.tail() == *_revs_loop_entry);
//             // proof_assert!((*_sloop_exit).tail() == **seq);
//             // proof_assert!((*_sloop_exit).tail() == (*_sloop_entry).tail());
//             // proof_assert!(Self::list(p, (*_sloop_exit).tail()));
//         }
//         proof_assert!(_seq0.subsequence(0, reverted_seq.len()) == *_seq0);
//         ghost!(**seq = reverted_seq.into_inner());
//         q
//     }
// }

// // #[ensures(Node::list(result.0, *result.1))]
// // #[ensures(result.1.len() == vec.view().len())]
// // #[ensures(forall<i: Int> 0 <= i && i < vec.view().len() ==> (*result.1)[i].val().elem == vec.view()[i])]
// // pub fn list_of_vector1<T>(mut vec: Vec<T>) -> (RawPtr<Node<T>>, Ghost<Seq<PtrOwn<Node<T>>>>) {
// //     //Takes possession of elements in the vector
// //     let (mut l, mut seq) = Node::empty();
// //     let _vec0 = snapshot!(vec);
// //     #[invariant(forall<i: Int>
// //         vec.view().len() <= i && i < _vec0.view().len() ==> seq[i - vec.view().len()].val().elem == _vec0.view()[i])]
// //     #[invariant(Node::list(l, *seq))]
// //     #[invariant(vec.view().len() + seq.len() == _vec0.view().len())]
// //     #[invariant(forall<i: Int> 0 <= i && i < vec.view().len() ==> vec.view()[i] == _vec0.view()[i])]
// //     #[invariant(inv(seq))]
// //     loop {
// //         if let Some(v) = vec.pop() {
// //             l = Node::cons(v, l, &mut seq);
// //         } else {
// //             break;
// //         }
// //     }
// //     (l, seq)
// // }

// // pub fn tr() {
// //     let v1 = creusot_contracts::vec![1, 5, 3];
// //     let (list1, mut _seq1) = list_of_vector1(v1.clone());
// //     assert!(*Node::nth(list1, 0, &_seq1) == 1);
// //     assert!(*Node::nth(list1, 1, &_seq1) == 5);
// //     assert!(*Node::nth(list1, 2, &_seq1) == 3);
// //     let l2 = Node::reverse_in_place(list1, &mut _seq1);
// //     assert!(*Node::nth(l2, 2, &_seq1) == 1);
// //     assert!(*Node::nth(l2, 1, &_seq1) == 5);
// //     assert!(*Node::nth(l2, 0, &_seq1) == 3);

// //     print!("ok");
// // }
