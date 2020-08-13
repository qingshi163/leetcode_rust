
use std::collections::BinaryHeap;
use super::utils::*;
use std::cmp::{ Ord, PartialOrd, Ordering };

type Node = Box<ListNode>;
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&other.val, &self.val)
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&other.val, &self.val)
    }
}

#[allow(dead_code)]
fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut node = &mut head;
    let mut pq: BinaryHeap<Node> = lists.into_iter().flatten().collect();
    while let Some(mut top) = pq.pop() {
        if let Some(next) = top.next.take() {
            pq.push(next);
        }
        if let Some(n) = node {
            n.next = Some(top);
            node = &mut n.next;
        } else {
            *node = Some(top);
        }
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23() {
        assert_eq!(merge_k_lists(vec![
            to_list(&[1,4,5]),
            to_list(&[1,3,4]),
            to_list(&[2,6]),
        ]), to_list(&[1,1,2,3,4,4,5,6]));
    }
}