use super::utils::ListNode;

#[allow(dead_code)]
fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut curr = &mut head;
    let mut carry = 0;
    let mut l1 = &l1;
    let mut l2 = &l2;
    while l1.is_some() || l2.is_some() {
        let mut sum = carry;
        if let Some(node) = l1 {
            sum += node.val;
            l1 = &node.next;
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = &node.next;
        }
        carry = sum / 10;
        *curr = Some(Box::new(ListNode::new(sum % 10)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils::to_list;

    #[test]
    fn test_2() {
        assert_eq!(add_two_numbers(to_list(&[2,4,3]), to_list(&[5,6,4])), to_list(&[7,0,8]));
    }
}