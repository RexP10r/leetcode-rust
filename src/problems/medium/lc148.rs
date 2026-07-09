#![allow(dead_code)]
use crate::utils::linked_list::ListNode;
struct Solution;
impl Solution {
    pub fn middle_and_break(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return None;
        }

        let mut slow: *mut Option<Box<ListNode>> = head;
        let mut fast: *mut Option<Box<ListNode>> = head;

        unsafe {
            while let Some(f_node) = (&mut *fast).as_mut() {
                if let Some(next_f) = f_node.next.as_mut() {
                    if next_f.next.is_none() {
                        break;
                    }
                    fast = &mut next_f.next as *mut Option<Box<ListNode>>;
                    slow = &mut (*slow).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                } else {
                    break;
                }
            }

            (*slow).as_mut().unwrap().next.take()
        }
    }
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let middle_head = Self::middle_and_break(&mut head);
        let l1 = Self::sort_list(head);
        let l2 = Self::sort_list(middle_head);
        Self::merge_sort(l1, l2)
    }
    pub fn merge_sort(
        mut head1: Option<Box<ListNode>>,
        mut head2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;
        while head1.is_some() && head2.is_some() {
            if head1.as_mut().unwrap().val <= head2.as_mut().unwrap().val {
                let mut node = head1.take().unwrap();
                head1 = node.next.take();
                curr.next = Some(node);
            } else {
                let mut node = head2.take().unwrap();
                head2 = node.next.take();
                curr.next = Some(node);
            }

            curr = curr.next.as_mut().unwrap();
        }
        if head1.is_some() {
            curr.next = head1;
        }
        if head2.is_some() {
            curr.next = head2;
        }
        dummy.next
    }
}
