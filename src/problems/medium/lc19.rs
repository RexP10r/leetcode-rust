#![allow(dead_code)]
struct Solution;
use crate::utils::linked_list::ListNode;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head.clone();

        let mut len = 0;
        {
            let mut cur = head.as_ref();
            while let Some(node) = cur {
                cur = node.next.as_ref();
                len += 1;
            }
        }

        let mut cur = &mut dummy;
        let steps = len - n;

        for _ in 0..steps {
            if let Some(ref mut node) = cur.next {
                cur = node;
            }
        }

        if let Some(mut node_to_delete) = cur.next.take() {
            cur.next = node_to_delete.next.take();
        }

        dummy.next
    }
}
