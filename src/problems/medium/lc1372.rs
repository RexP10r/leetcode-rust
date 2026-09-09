#![allow(dead_code)]
use crate::utils::bst::TreeNode;
struct Solution;

struct DP {
    pub left_path: i32,
    pub right_path: i32,
    pub res: i32,
}
impl Default for DP {
    fn default() -> Self {
        Self {
            left_path: -1,
            right_path: -1,
            res: -1,
        }
    }
}
impl DP {
    fn get_max_path(left: Self, right: Self) -> Self {
        Self {
            left_path: left.right_path + 1,
            right_path: right.left_path + 1,
            res: vec![
                left.right_path + 1,
                right.left_path + 1,
                left.res,
                right.res,
            ]
            .iter()
            .copied()
            .max()
            .unwrap(),
        }
    }
}
fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> DP {
    match root {
        None => DP::default(),
        Some(node) => {
            let borrowed_node = node.borrow();
            let (left, right) = (
                dfs(borrowed_node.left.as_ref()),
                dfs(borrowed_node.right.as_ref()),
            );
            DP::get_max_path(left, right)
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dfs(root.as_ref()).res
    }
}
