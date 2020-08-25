/*
Given a singly linked list L: L0→L1→…→Ln-1→Ln,
reorder it to: L0→Ln→L1→Ln-1→L2→Ln-2→…

You may not modify the values in the list's nodes, only nodes itself may be changed.

Example 1:

Given 1->2->3->4, reorder it to 1->4->2->3.

Example 2:

Given 1->2->3->4->5, reorder it to 1->5->2->4->3.


 */

use super::utils::*;
use std::collections::VecDeque;
#[allow(dead_code)]
fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut seq: VecDeque<Box<ListNode>> = VecDeque::new();
    let mut curr = head.take();
    while let Some(mut node) = curr.take() {
        curr = node.next.take();
        seq.push_back(node);
    }
    *head = seq.pop_front();
    if let Some(mut prev) = head.as_mut() {
        loop {
            if let Some(node) = seq.pop_back() {
                prev.next = Some(node);
                prev = prev.next.as_mut().unwrap();
            } else {
                break;
            }
            if let Some(node) = seq.pop_front() {
                prev.next = Some(node);
                prev = prev.next.as_mut().unwrap();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut l = to_list(&[1, 2, 3, 4]);
        reorder_list(&mut l);
        assert_eq!(l, to_list(&[1, 4, 2, 3]));
    }
}
