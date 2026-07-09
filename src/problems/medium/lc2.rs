#![allow(dead_code)]
struct Solution;
use crate::utils::linked_list::ListNode;
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut remainder: i32 = 0;
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;

        while l1.is_some() || l2.is_some() || remainder > 0 {
            let total_val;
            {
                let v1 = l1.as_deref().map_or(0, |n| n.val);
                let v2 = l2.as_deref().map_or(0, |n| n.val);
                total_val = v1 + v2 + remainder;
            }
            remainder = total_val / 10;

            curr.next = Some(Box::new(ListNode::new(total_val % 10)));
            curr = curr.next.as_mut().unwrap();

            if let Some(l1_node) = l1 {
                l1 = l1_node.next
            }
            if let Some(l2_node) = l2 {
                l2 = l2_node.next
            }
        }
        dummy.next
    }
}
