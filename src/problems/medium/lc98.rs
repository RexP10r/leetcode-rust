#![allow(dead_code)]
struct Solution;
use crate::utils::bst::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn is_valid_node(root: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i64>) -> bool {
        let node = match root {
            Some(n) => n,
            None => {
                return true;
            }
        };
        if !Self::is_valid_node(&node.borrow().left, prev) {
            return false;
        }
        let node_val = node.borrow().val as i64;
        if let Some(prev_val) = *prev {
            if node_val <= prev_val {
                return false;
            }
        }
        *prev = Some(node_val);
        Self::is_valid_node(&node.borrow().right, prev)
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev: Option<i64> = None;
        Self::is_valid_node(&root, &mut prev)
    }
}
