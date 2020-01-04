use super::utils::ListNode;

#[allow(dead_code)]
fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut dummy;
    let mut l1 = &l1;
    let mut l2 = &l2;
    loop {
        let val = if match (l1.is_some(), l2.is_some()) {
            (false, false) => break,
            (true,  false) => true,
            (false, true ) => false,
            (true,  true ) => l1.as_ref().unwrap().val < l2.as_ref().unwrap().val
        } {
            let val = l1.as_ref().unwrap().val;
            l1 = &l1.as_ref().unwrap().next;
            val
        } else {
            let val = l2.as_ref().unwrap().val;
            l2 = &l2.as_ref().unwrap().next;
            val
        };
        curr.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils::to_list;

    #[test]
    fn test_21() {
        assert_eq!(merge_two_lists(
            to_list(&[1,2,4]),
            to_list(&[1,3,4])
        ), to_list(&[1,1,2,3,4,4]));
    }
}