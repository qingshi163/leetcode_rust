
use super::utils::*;
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

// Reverse node between head and tail, link the tail as the last node
// Return new head
fn reverse(mut head: Option<Box<ListNode>>, mut tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    while let Some(mut node) = head.take() {
        head = node.next.take();
        node.next = tail.take();
        tail = Some(node);
    }
    tail
}

#[allow(dead_code)]
fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut curr = &mut head;
    for _ in 0..k {
        match curr {
            None => return head,
            Some(node) => curr = &mut node.next
        }
    }
    let tail = curr.take();
    reverse(head, reverse_k_group(tail, k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_25() {
        assert_eq!(reverse(to_list(&[1,2,3,4,5]), to_list(&[6,7,8])), to_list(&[5,4,3,2,1,6,7,8]));
        assert_eq!(reverse_k_group(to_list(&[1,2,3,4,5]), 2), to_list(&[2,1,4,3,5]));
        assert_eq!(reverse_k_group(to_list(&[1,2,3,4,5]), 3), to_list(&[3,2,1,4,5]));
    }
}