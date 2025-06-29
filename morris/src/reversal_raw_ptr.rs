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

impl<T> Node<T> {
    #[predicate]
    #[variant(perm_seq.len())]
    fn list(l: RawPtr<Self>, perm_seq: Seq<PtrOwn<Node<T>>>) -> bool {
        //On n'aura pas vraiment besoin de l puisque on suppose que perm_seq ne peut pas etre autre chose que la liste
        //des permissions de l
        pearlite! {
            if l.is_null_logic() {
                perm_seq.len() == 0
            } else {
                 if perm_seq.len() > 0 {
                    let ptr = perm_seq[0].ptr();
                    l == ptr && Self::list(perm_seq[0].val().next, perm_seq.tail())
                } else {
                    false
                }
            }
        }
    }

    #[ensures(Self::list(result.0, *result.1))]
    #[ensures(result.0.is_null_logic())]
    pub fn empty() -> (RawPtr<Self>, Ghost<Seq<PtrOwn<Node<T>>>>) {
        (ptr::null(), Seq::new())
    }

    #[trusted]
    #[requires(Self::list(l, **seq))]
    #[ensures(Self::list(result,  *^seq))]
    #[ensures(forall<i:Int> 0 <= i && i < (^seq).tail().len() ==> seq[i] == (^seq).tail()[i])]
    #[ensures((^seq)[0].val().elem == e)]
    #[ensures((^seq).len() == seq.len() + 1)]
    pub fn cons(e: T, l: RawPtr<Self>, seq: &mut Ghost<Seq<PtrOwn<Node<T>>>>) -> RawPtr<Self> {
        // let ee = snapshot!(e);
        let (raw, own) = PtrOwn::new(Node { elem: e, next: l });

        let seq2 = snapshot!(**seq);
        ghost!(seq.push_front_ghost(own.into_inner()));
        proof_assert!(*seq2 == seq.tail());

        raw
    }

    #[trusted]
    #[requires(Self::list(p, **seq))]
    #[requires(0 <= nth@ && nth@ < seq.len() )]
    #[ensures(seq[nth@].val().elem == *result)]
    pub fn nth(mut p: RawPtr<Self>, nth: i128, seq: &Ghost<Seq<PtrOwn<Node<T>>>>) -> &T {
        //requires nth >= 0
        let mut i = 0;
        //let mut seq_taililng = snapshot!(**seq);
        proof_assert!(**seq == seq.subsequence(0, seq.len()));
        #[invariant(0 <= i@ && i@ <= nth@)]
        #[invariant(Self::list(p, seq.subsequence(i@, seq.len())))]
        loop {
            //je ne comprends pas pourquoi il n'arirve pas à prouver les deux assertions en bas alors que c'est trivial
            // hypothèse: snapshot! n'est bon pour tracker la valeur de seq meme si c'est une valeur immutable
            //proof_assert!(seq_taililng[0] == seq[i@]);
            //proof_assert!(Self::list(p, *seq_taililng));
            let rw = unsafe {
                PtrOwn::as_ref(p, ghost!(seq.get_ghost(Int::new(i).into_inner()).unwrap()))
            };

            if i == nth {
                return &rw.elem;
            }

            p = rw.next;
            proof_assert!(seq.subsequence(i@, seq.len()).tail() == seq.subsequence(i@+1, seq.len()));
            i += 1;
            //seq_taililng = snapshot!((*seq_taililng).tail());
        }
    }

    #[predicate]
    pub fn contains(s: Seq<PtrOwn<Node<T>>>, x: T) -> bool
    where
        T: Sized, // TODO: don't require this (problem: uses index)
    {
        pearlite! { exists<i: Int> 0 <= i &&  i < s.len() && s[i].val().elem == x }
    }

    #[predicate]
    pub fn reverse(seq: Seq<PtrOwn<Node<T>>>, other: Seq<PtrOwn<Node<T>>>, lb: Int, lh: Int) -> bool
    where
        T: Sized, // TODO: don't require this (problem: uses index)
    {
        pearlite! {
             forall<i: Int>
            lb <= i && i < lh
            ==> seq[i].val().elem == other[other.len() - i - 1].val().elem
        }
    }

    #[trusted]
    #[logic]
    #[requires(Self::reverse(seq0.subsequence(0, rev_seq.len()), *rev_seq, 0, rev_seq.len()))]
    #[requires(*seq == seq0.subsequence(rev_seq.len(), seq0.len()))]
    #[ensures(rev_seq.len() + seq.len() == seq0.len())]
    fn disjunciton_lemma(
        seq0: &Seq<PtrOwn<Node<T>>>,
        seq: &Seq<PtrOwn<Node<T>>>,
        rev_seq: &Seq<PtrOwn<Node<T>>>,
    ) {
        proof_assert!(rev_seq.len() == seq0.subsequence(0, rev_seq.len()).len());
        proof_assert!(
            seq0.subsequence(0, rev_seq.len())
                .concat(seq0.subsequence(rev_seq.len(), seq0.len()))
                == *seq0
        );
    }

    #[requires(Self::list(p, **seq))]
    #[ensures(Self::list(result, *^seq))]
    #[ensures(seq.len() == (^seq).len() && Self::reverse(**seq, *^seq, 0, seq.len()))]
    //stabilité par inversion
    //#[ensures(forall<i: Int> 0 <= i && i < seq.len() ==> exists<j: Int> 0 <= j && j < (^seq).len() && (seq[j].val().elem == (^seq)[i].val().elem))]

    // #[ensures(seq.len() == (^seq).len())]
    // #[ensures(forall<e: T> Self::contains(**seq, e) ==> Self::contains(*^seq, e))]
    pub fn reverse_in_place(
        mut p: RawPtr<Self>,
        seq: &mut Ghost<Seq<PtrOwn<Node<T>>>>,
    ) -> RawPtr<Self> {
        //requires p n'est pas un lasso
        snapshot! {
            let _ = Seq::<T>::ext_eq;
        };
        let mut q: *const Node<T> = ptr::null();
        let mut reverted_seq = Seq::new();
        let seq0 = snapshot!(**seq);

        #[invariant(Self::list(q, *reverted_seq))]
        #[invariant(Self::list(p, **seq))]
        #[invariant(Self::reverse(seq0.subsequence(0, reverted_seq.len()), *reverted_seq, 0, reverted_seq.len()))]
        //Question!!!!!!!!!! Ican either keep this invariant which proves everything, or remove it and prove disjunction_lemma
        //which makes the code cleaner.
        //#[invariant(reverted_seq.len() + seq.len() == seq0.len())]
        #[invariant(**seq == seq0.subsequence(reverted_seq.len(), seq0.len()))]
        #[invariant(inv(seq))]
        #[invariant(inv(reverted_seq))]
        while !p.is_null() {
            snapshot!(Self::disjunciton_lemma(&*seq0, &**seq, &*reverted_seq));
            let sloop_entry = snapshot!(**seq);
            let revs_loop_entry = snapshot!(*reverted_seq);
            let p2 =
                unsafe { PtrOwn::as_mut(p, ghost!(seq.get_mut_ghost(*ghost!(0int)).unwrap())) };
            let next = p2.next;
            p2.next = q;
            q = p;
            p = next;
            let sloop_exit = snapshot!(**seq);

            ghost!((*reverted_seq).push_front_ghost(seq.pop_front_ghost().unwrap()));

            //a0156: Assertion used to prove invariant #1 (we can remove it and use use_th seq.FreeMonoid instead)
            proof_assert!(reverted_seq.tail() == *revs_loop_entry);

            //Hypothesis: invariant(Self::list (p, **seq))
            // We need to add to the hypothesis the fac that the tail of the previous seq is the new seq
            //a1369
            proof_assert!((*sloop_exit).tail() == **seq);

            //In order to proof the last assertion, we need the following assertion
            //It esnures that seq.tail() didn't change between the beginig of the loop and the end, what ensures the stability of our invariant
            //a7070
            proof_assert!((*sloop_exit).tail() == (*sloop_entry).tail());

            //this should be enough to prove #[invariant(Self::list (p, **seq))], whith using the latter, creusot proves well the remaining invariant about q
            //proof_assert!(Self::list(p, (*snap2).tail()));
            //a1313
            proof_assert!(Self::list(p, (*sloop_exit).tail()));
            // ==> invariant #1 checks for iteration n+1
        }
        snapshot!(Self::disjunciton_lemma(&*seq0, &**seq, &*reverted_seq));
        //Pour montrer ensures#1 (ensures(seq.len() == (^seq).len() && Self::reverse(**seq, *^seq, 0, seq.len())))
        //a4224
        proof_assert!(seq0.subsequence(0, reverted_seq.len()) == *seq0);
        ghost!(**seq = reverted_seq.into_inner());
        q
    }
}

#[trusted]
#[ensures(Node::list(result.0, *result.1))]
#[ensures(result.1.len() == vec.view().len())]
#[ensures(forall<i: Int> 0 <= i && i < vec.view().len() ==> (*result.1)[i].val().elem == vec.view()[i])]
pub fn list_of_vector1<T>(mut vec: Vec<T>) -> (RawPtr<Node<T>>, Ghost<Seq<PtrOwn<Node<T>>>>) {
    //Takes possession of elements in the vector
    let (mut l, mut seq) = Node::empty();
    let vec0 = snapshot!(vec);
    #[invariant(forall<i: Int>
        vec.view().len() <= i && i < vec0.view().len() ==> seq[i - vec.view().len()].val().elem == vec0.view()[i])]
    #[invariant(Node::list(l, *seq))]
    #[invariant(vec.view().len() + seq.len() == vec0.view().len())]
    #[invariant(forall<i: Int> 0 <= i && i < vec.view().len() ==> vec.view()[i] == vec0.view()[i])]
    #[invariant(inv(seq))]
    loop {
        //let vec1 = snapshot!(vec);
        //let seq1 = snapshot!(seq);
        //proof_assert!(vec1.view().len() > 0 ==> vec1.view()[vec1.view().len()-1] == vec0.view()[vec1.view().len()-1]);

        if let Some(v) = vec.pop() {
            //let vv = snapshot!(v);
            l = Node::cons(v, l, &mut seq);
            //proof_assert!(vec1.view().len() == vec.view().len() + 1);
            //proof_assert!(seq[0].val().elem == *vv && seq.tail() == **seq1);
            //proof_assert!(forall<i: Int>
            //  vec1.view().len() <= i && i < vec0.view().len() ==> seq1[i - vec1.view().len()].val().elem == vec0.view()[i]);
            //proof_assert!(vec1.view() == vec.view().push_back(*vv));
            //proof_assert!(vec1.view()[vec.view().len()] == vec0.view()[vec.view().len()]);
            //proof_assert!(seq[0].val().elem == vec0.view()[vec.view().len()]);
        } else {
            break;
        }
    }
    (l, seq)
}

#[trusted]
pub fn list_of_vector2<T: Clone>(vec: &Vec<T>) -> (RawPtr<Node<T>>, Ghost<Seq<PtrOwn<Node<T>>>>) {
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
