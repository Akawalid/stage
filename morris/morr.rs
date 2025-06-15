type NodeRef<T> = Option<Rc<RefCell<TreeNode<T>>>>;

struct TreeNode<T> {
    val: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

//Deuxième représentation:

// enum Tree<T> {
//     Empty,
//     Node {
//         val: T,
//         left: Rc<RefCell<Tree<T>>>,
//         right: Rc<RefCell<Tree<T>>>,
//     },
// }

// impl<T> Tree<T> {
//     fn new(val: T) -> Rc<RefCell<Self>> {
//         Rc::new(RefCell::new(Tree::Node {
//             val,
//             left: Rc::new(RefCell::new(Tree::Empty)),
//             right: Rc::new(RefCell::new(Tree::Empty)),
//         }))
//     }
// }

fn morris_inorder_traversal<T>(root: NodeRef<T>) {
    let mut t = root;

    while !t.is_none() {
        if t.clone().unwrap().borrow().left.is_none() {
            //println!("{:?}", t.clone().unwrap().borrow().val);
            t = t.clone().unwrap().borrow().right.clone();
        } else {
            let mut q = t.clone().unwrap().borrow().left.clone().unwrap();
            loop {
                if q.borrow().right.is_none()
                    || Rc::ptr_eq(q.borrow().right.as_ref().unwrap(), &t.clone().unwrap())
                {
                    break;
                }
                let next = q.borrow().right.clone().unwrap();
                q = next;
            }

            let mut mutable_ref = q.borrow_mut();
            if mutable_ref.right.is_none() {
                mutable_ref.right = Some(t.clone().unwrap());
                t = t.clone().unwrap().borrow().left.clone();
            } else {
                mutable_ref.right = None;
                //println!("{:?}", t.clone().unwrap().borrow().val);
                t = t.clone().unwrap().borrow().right.clone();
            }
        }
    }
}
