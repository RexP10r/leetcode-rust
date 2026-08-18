#![allow(dead_code)]
use crate::utils::bst::TreeNode;
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;
fn dfs<'a>(root: &Option<Rc<RefCell<TreeNode>>>, max_way: &'a mut i32) -> i32 {
    if let Some(node) = root {
        let node_ref = node.borrow();
        let left_gain = max(dfs(&node_ref.left, max_way), 0);
        let right_gain = max(dfs(&node_ref.right, max_way), 0);
        *max_way = max(left_gain + right_gain + node_ref.val, *max_way);
        node_ref.val + max(left_gain, right_gain)
    } else {
        0
    }
}
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_way: i32 = i32::MIN;
        let _ = dfs(&root, &mut max_way);

        max_way
    }
}
