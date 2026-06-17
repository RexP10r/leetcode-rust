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
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy.next;

        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                let mut next_node = list1.take().unwrap();
                list1 = next_node.next.take();
                *tail = Some(next_node);
            } else {
                let mut next_node = list2.take().unwrap();
                list2 = next_node.next.take();
                *tail = Some(next_node);
            }
            tail = &mut tail.as_mut().unwrap().next;
        }

        *tail = list1.or(list2);

        dummy.next
    }
}
