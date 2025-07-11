extern crate creusot_contracts;
//in this file, I put all the exercises I have done during the internship.
//pub mod reversal_raw_ptr;
pub mod lasso;
// type NodeRef<T> = Option<Rc<Comb<T>>>;
// struct Comb<T> {
//     pcell: PCell<TreeNode<T>>,
//     id: Int,
// }
// struct TreeNode<T> {
//     val: T,
//     left: NodeRef<T>,
//     right: NodeRef<T>,
// }

// fn new<T>(val: T, id: Int) -> (NodeRef<T>, Ghost<PCellOwn<TreeNode<T>>>) {
//     let (pcell, perm) = PCell::new(TreeNode {
//         val,
//         left: None,
//         right: None,
//     });

//     (Some(Rc::new(Comb { pcell, id })), perm)
// }

// fn morris_inorder_traversal<T>(root: NodeRef<T>, mut seq: Seq<Ghost<PCellOwn<TreeNode<T>>>>) {
//     let mut t = root;

//     while !t.is_none() {
//         if unsafe {
//             let perm = seq.get_ghost(t.clone().unwrap().id).unwrap().borrow();
//             t.clone().unwrap().pcell.borrow(perm).left.is_none()
//         } {
//             //println!("{:?}", t.clone().unwrap().borrow().val);
//             unsafe {
//                 let perm = seq.get_ghost(t.clone().unwrap().id).unwrap().borrow();
//                 t = t.clone().unwrap().pcell.borrow(perm).right.clone();
//             }
//         } else {
//             let mut q = unsafe {
//                 let perm = seq.get_ghost(t.clone().unwrap().id).unwrap().borrow();
//                 t.clone().unwrap().pcell.borrow(perm).left.clone().unwrap()
//             };

//             loop {
//                 if unsafe {
//                     let perm = seq.get_ghost(t.clone().unwrap().id).unwrap().borrow();
//                     q.pcell.borrow(perm).right.is_none()
//                 } || unsafe {
//                     let perm = seq.get_ghost(t.clone().unwrap().id).unwrap().borrow();
//                     Rc::ptr_eq(
//                         q.pcell.borrow(perm).right.as_ref().unwrap(),
//                         &t.clone().unwrap(),
//                     )
//                 } {
//                     break;
//                 }
//                 let next;
//                 let perm = seq.get_ghost(t.clone().unwrap().id).unwrap().borrow();
//                 unsafe { next = q.pcell.borrow(perm).right.clone().unwrap() };
//                 q = next;
//             }
//             let perm = seq.get_mut_ghost(q.clone().id).unwrap().borrow_mut();

//             let mutable_ref = unsafe { q.pcell.borrow_mut(perm) };
//             if mutable_ref.right.is_none() {
//                 mutable_ref.right = Some(t.clone().unwrap());
//                 let perm = seq.get_ghost(t.clone().unwrap().id).unwrap().borrow();
//                 unsafe {
//                     t = t.clone().unwrap().pcell.borrow(perm).left.clone();
//                 };
//             } else {
//                 mutable_ref.right = None;
//                 //println!("{:?}", t.clone().unwrap().borrow().val);
//                 let perm = seq.get_ghost(t.clone().unwrap().id).unwrap().borrow();
//                 unsafe {
//                     t = t.clone().unwrap().pcell.borrow(perm).right.clone();
//                 }
//             }
//         }
//     }
// }

// #[predicate]
// #[variant(seq.len() - i)]
// #[requires(i >= 0 && i < seq.len())]
// fn in_heap(seq: Seq<i32>, i: Int, j: Int) -> bool {
//     pearlite! {(i == j || (2 * i + 1 < seq.len() && in_heap(seq, 2 * i + 1, j)) || (2 * i + 2 < seq.len() && in_heap(seq, 2 * i + 2, j)))}
// }

// #[predicate]
// #[variant(seq.len() - i)]
// #[requires(i >= 0 && i < seq.len())]
// fn heap(seq: Seq<i32>, i: Int) -> bool {
//     pearlite! {2 * i + 2 < seq.len() ==> seq[i]@ >= seq[2 * i + 2]@ &&  heap(seq, 2 * i + 2) && 2 * i + 1 < seq.len() ==> seq[i]@ >= seq[2 * i + 1]@ && heap(seq, 2 * i + 1)}
// }

// #[predicate]
// fn sorted(arr: Seq<i32>, lb: Int, hb: Int) -> bool {
//     pearlite! {
//         forall<i: Int, j: Int> j >= lb && j < i && i < hb ==> arr[j]@ <= arr[i]@
//     }
// }

// #[requires(n@ <= arr@.len())]
// #[requires(2 * arr@.len() + 2 < usize::MAX@)]
// #[requires(i@ < n@)]
// #[requires(heap(arr@, 2 * i@ + 1))]
// #[requires(heap(arr@, 2 * i@ + 2))]
// #[ensures(heap((^arr)@, i@))]
// #[ensures(arr@.permutation_of((^arr)@))]
// fn heapify(arr: &mut [i32], n: usize, i: usize) {
//     let mut largest = i;
//     let left = 2 * i + 1;
//     let right = 2 * i + 2;

//     if left < n && arr[left] > arr[largest] {
//         largest = left;
//     }
//     if right < n && arr[right] > arr[largest] {
//         largest = right;
//     }

//     if largest != i {
//         arr.swap(i, largest);
//         heapify(arr, n, largest);
//     }
// }

// //#[requires(2 * arr@.len() + 2 < usize::MAX@)]
// #[ensures(sorted((^arr)@, 0, (^arr)@.len()))]
// #[ensures(arr@.permutation_of((^arr)@))]
// fn heap_sort(arr: &mut [i32]) {
//     let n = arr.len();
//     let arr0 = snapshot!(arr);

//     proof_assert!(2 *  arr@.len() / 2 + 1 >=  arr@.len());
//     proof_assert!(2 *  arr@.len() / 2 + 2 < arr@.len() ==> arr[arr@.len() / 2]@ >= arr[2 *  arr@.len() / 2 + 2]@ &&  heap(arr@, 2 *  arr@.len() / 2 + 2)
//     && 2 *  arr@.len() / 2 + 1 < arr@.len() ==> arr[ arr@.len() / 2]@ >= arr[2 *  arr@.len() / 2 + 1]@ && heap(arr@, 2 *  arr@.len() / 2 + 1));
//     proof_assert!(heap(arr@, arr@.len() / 2));
//     #[invariant(heap(arr@, n@ / 2 - produced.len()))]
//     #[invariant(arr@.permutation_of(arr0@))]
//     for i in (0..n / 2).rev() {
//         heapify(arr, n, i);
//     }

//     #[invariant(sorted(arr@, produced.len() + 1, n@))]
//     #[invariant(arr@.permutation_of(arr0@))]
//     for i in (1..n).rev() {
//         arr.swap(0, i);
//         heapify(arr, i, 0);
//     }
// }

// extern crate creusot_contracts;
// use creusot_contracts::*;

// #[predicate]
// fn sorted(arr: Seq<i32>, lb: Int, hb: Int) -> bool {
//     pearlite! {
//         forall<i: Int, j: Int> j >= lb && j < i && i < hb ==> arr[j]@ <= arr[i]@
//     }
// }

// #[ensures(sorted((^arr)@, 0, (^arr)@.len()))]
// #[ensures(arr@.permutation_of((^arr)@))]
// fn insertion_sort(arr: &mut [i32]) {
//     let original = snapshot!(arr);
//     let len = arr.len();
//     #[invariant(original@.permutation_of(arr@))]
//     #[invariant(sorted(arr@, 0, produced.len()))]
//     #[invariant(arr@.len() == len@)]
//     for i in 1..len {
//         let mut j = i;
//         proof_assert!(i@ == produced.len());
//         #[invariant(sorted(arr@, j@,  i@+1))]
//         #[invariant(sorted(arr@, 0,  j@))]
//         #[invariant(arr@.len() == len@)]
//         #[invariant(original@.permutation_of(arr@))]
//         #[invariant(j <= i)]
//         while j > 0 && arr[j - 1] > arr[j] {
//             arr.swap(j - 1, j);
//             j -= 1;
//         }
//         proof_assert!(sorted(arr@, j@,  i@+1) && sorted(arr@, 0,  j@));
//         proof_assert!(j@ <= 0 && sorted(arr@, j@,  i@+1) && sorted(arr@, 0,  j@) ==> sorted(arr@, 0,  i@+1));
//         proof_assert!(arr[j@ - 1]@ <= arr[j]@ && sorted(arr@, j@,  i@+1) && sorted(arr@, 0,  j@) ==> sorted(arr@, 0,  i@+1));
//         proof_assert!(sorted(arr@, 0,  i@+1));
//         proof_assert!(sorted(arr@, 0,  i@+1) ==> sorted(arr@, 0,  produced.len()+1));
//     }
// }

// extern crate creusot_contracts;
// use creusot_contracts::*;

// // Prove that this function implements the sum of the first n natural numbers.
// // Hint: there exists a closed form of this sum

// //#[requires(n@ * (n@ + 1) <= 2 * u32::MAX@)]
// #[requires(n@ < 1000)]
// #[ensures(result@ == n@ * (n@ + 1) / 2)]
// pub fn sum_first_n(n: u32) -> u32 {
//     let mut sum = 0;
//     let mut i = 0;

//     #[invariant(2 * sum@ == i@ * (i@ + 1))]
//     #[invariant(i@ < n@ + 1)]
//     while i < n {
//         i += 1;
//         sum += i;
//     }
//     sum
// }

// extern crate creusot_contracts;
// use creusot_contracts::*;

// // Implement and prove the Euclidean division
// #[requires(u@ >=0 && b@ > 0)]
// #[ensures(u@ == b@ * (*result.1)@ + result.0@)]
// #[ensures(result.0@ >=0 && result.0@ < b@)]
// fn divide(u: i64, b: i64) -> (i64, Ghost<i64>) {
//     if u - b < 0 {
//         (u, ghost!(0))
//     } else {
//         let result = divide(u - b, b);
//         (result.0, ghost!(*result.1 + 1))
//     }
// }

// extern crate creusot_contracts;
// use creusot_contracts::*;

// // Prove the following:
// // 1. If we return Some(i) it is the first index containing `tgt`
// // 2. If we return None, then there are no indices containing `tgt`
// #[ensures(
//     match result {
//         Some(i) => forall<j: Int> 0 <= j && j < i@ ==> v@[j] != *tgt,
//         None => forall<j: Int> 0 <= j && j < v@.len() ==> v@[j] != *tgt,
//     }
// )]

// fn search<T: Ord + DeepModel>(v: &[T], tgt: &T) -> Option<usize> {
//     let mut i = 0;

//     #[invariant(forall<j: Int> j >= 0 && j < i@ ==> v@[j] != *tgt)]
//     while i < v.len() {
//         if v[i] == *tgt {
//             return Some(i);
//         }

//         i += 1
//     }

//     return None;
// }

// extern crate creusot_contracts;
// use creusot_contracts::{logic::Int, *};

// // Prove that after the call to this function the vector only contains zeroes
// // Also show that no elements were added or removed

// #[ensures(forall<i: Int> i >= 0 && i < (^v)@.len() ==> (^v)@[i] == 0u32)]
// pub fn all_zero(v: &mut Vec<u32>) {
//     let mut i = 0;
//     let old_v = snapshot! { v };

//     #[invariant(forall<j: Int> j >= 0 && j < i@ ==> v@[j] == 0u32)]
//     while i < v.len() {
//         v[i] = 0;
//         i += 1;
//     }
// }
