#![allow(dead_code)]
struct Solution;
use crate::utils::linked_list::ListNode;
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        {
            let mut cur = head.as_ref();
            while let Some(node) = cur {
                cur = node.next.as_ref();
                len += 1;
            }
        }
        if len == n {
            return head.and_then(|n| n.next);
        }

        let mut cur = head.as_mut();
        let steps = len - n - 1;

        for _ in 0..steps {
            if let Some(node) = cur {
                cur = node.next.as_mut();
            }
        }

        if let Some(node) = cur {
            let next_node = node.next.take();
            node.next = next_node.and_then(|n| n.next);
        }

        head
    }
}
