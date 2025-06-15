use std::cell::RefCell;
use std::rc::Rc;

mod reversal;

type NodeRef<T> = Option<Rc<RefCell<TreeNode<T>>>>;

#[derive(Debug)]
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

fn morris_inorder_traversal<T: std::fmt::Debug>(root: NodeRef<T>) {
    let mut t = root;

    while !t.is_none() {
        if t.clone().unwrap().borrow().left.is_none() {
            // Visit the node
            println!("{:?}", t.clone().unwrap().borrow().val);
            t = t.clone().unwrap().borrow().right.clone();
        } else {
            // Find the inorder predecessor
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
                // Thread it
                mutable_ref.right = Some(t.clone().unwrap());
                t = t.clone().unwrap().borrow().left.clone();
            } else {
                // Unthread
                mutable_ref.right = None;
                println!("{:?}", t.clone().unwrap().borrow().val);
                t = t.clone().unwrap().borrow().right.clone();
            }
        }
    }
}

fn main() {
    let root = TreeNode::new(2);
    let node2 = TreeNode::new(1);
    let node3 = TreeNode::new(4);
    let node4 = TreeNode::new(5);
    let node5 = TreeNode::new(3);

    root.borrow_mut().left = Some(node2.clone());
    root.borrow_mut().right = Some(node3.clone());
    node3.borrow_mut().right = Some(node4.clone());
    node3.borrow_mut().left = Some(node5.clone());

    morris_inorder_traversal(Some(root));
}
