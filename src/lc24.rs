
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
#[allow(dead_code)]
fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.map(|mut a| {
        match a.next.take() {
            None => a,
            Some(mut b) => {
                a.next = swap_pairs(b.next.take());
                b.next = Some(a);
                b
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24() {
        assert_eq!(swap_pairs(to_list(&[1,2,3,4])), to_list(&[2,1,4,3]));
    }
}