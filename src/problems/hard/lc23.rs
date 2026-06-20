#![allow(dead_code)]
struct Solution;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut priority_queue = BinaryHeap::new();
        for (i, list) in lists.iter().enumerate() {
            if let Some(node) = list {
                priority_queue.push(Reverse((node.val, i)));
            }
        }
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        while let Some(Reverse((_, i))) = priority_queue.pop() {
            if let Some(mut head) = lists[i].take() {
                lists[i] = head.next.take();
                if let Some(next_node) = &lists[i] {
                    priority_queue.push(Reverse((next_node.val, i)));
                } 
                current.next = Some(head);
                current = current.next.as_mut().unwrap();
            }
            
        };
        dummy.next
    }
}
