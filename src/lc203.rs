use super::utils::ListNode;

#[allow(dead_code)]
fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut curr = &mut head;
    while curr.is_some() {
        if curr.as_ref().unwrap().val == val {
            *curr = curr.take().unwrap().next;
        } else {
            curr = &mut curr.as_mut().unwrap().next;
        }
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils::to_list;

    #[test]
    fn test_203() {
        assert_eq!(remove_elements(to_list(&[1,2,6,3,4,5,6]), 6), to_list(&[1,2,3,4,5]));
    }
}