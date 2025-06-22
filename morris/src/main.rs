use creusot_contracts::vec;
use morris::reversal_raw_ptr::*;
fn main() {
    let v1 = vec![1, 5, 3];
    let (list1, mut _seq1) = list_of_vector1(v1.clone());
    assert!(*Node::nth(list1, 0, &_seq1) == 1);
    assert!(*Node::nth(list1, 1, &_seq1) == 5);
    assert!(*Node::nth(list1, 2, &_seq1) == 3);
    let l2 = Node::reverse_in_place(list1, &mut _seq1);
    assert!(*Node::nth(l2, 2, &_seq1) == 1);
    assert!(*Node::nth(l2, 1, &_seq1) == 5);
    assert!(*Node::nth(l2, 0, &_seq1) == 3);
    //Si on enlève i32, creusot n'acceptera pas les types polymorphes

    // let v2: Vec<i32> = vec![];
    // let (list1, _seq1) = list_of_vector1(v2.clone());
    //assert!(list1 == List::new());

    print("ok");
}
