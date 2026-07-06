#![allow(dead_code)]
struct Solution;
use crate::utils::bst::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn is_mirror(r1: Option<Rc<RefCell<TreeNode>>>, r2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (r1, r2) {
            (None, None) => true,
            (Some(n1), Some(n2)) => {
                let nr1 = n1.borrow();
                let nr2 = n2.borrow();
                if nr1.val != nr2.val {
                    return false;
                }
                let l1 = nr1.left.clone();
                let l2 = nr2.left.clone();
                let r1 = nr1.right.clone();
                let r2 = nr2.right.clone();

                Self::is_mirror(l1, r2) && Self::is_mirror(r1, l2)
            }
            _ => false,
        }
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node_ref = node.borrow();
                Self::is_mirror(node_ref.left.clone(), node_ref.right.clone())
            }
        }
    }
}
