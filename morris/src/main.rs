use creusot_contracts::ptr_own::PtrOwn;
use creusot_contracts::vec;
use creusot_contracts::*;
use morris::reversal_raw_ptr::*;
fn main() {
    let v1 = vec![1, 5, 3];
    let (list1, mut _seq1) = list_of_vector1(v1.clone());
    assert!(*Node::nth(list1, 0, _seq1.borrow()) == 1);
    assert!(*Node::nth(list1, 1, _seq1.borrow()) == 5);
    assert!(*Node::nth(list1, 2, _seq1.borrow()) == 3);
    let l2 = Node::reverse_in_place(list1, _seq1.borrow_mut());
    assert!(*Node::nth(l2, 2, _seq1.borrow()) == 1);
    assert!(*Node::nth(l2, 1, _seq1.borrow()) == 5);
    assert!(*Node::nth(l2, 0, _seq1.borrow()) == 3);
    //Si on enlève i32, creusot n'acceptera pas les types polymorphes

    // let v2: Vec<i32> = vec![];
    // let (list1, _seq1) = list_of_vector1(v2.clone());
    //assert!(list1 == List::new());

    print!("ok");

    // if false {
    //     let (m1, mut seq) = Node::<i128>::empty();
    //     let m2 = Node::<i128>::cons(1, m1, &mut seq);
    //     let m3 = Node::<i128>::cons(0, m2, &mut seq);
    //     let mutm2 = unsafe { PtrOwn::as_mut(m2, ghost!(seq.get_mut_ghost(1int).unwrap())) };
    //     mutm2.next = m3;
    //     /*
    //     | m3 | -> | m2 | -> || m1
    //       ^         ''
    //       | _______ |
    //      */
    // }
}
