use super::utils::ListNode; 
#[allow(dead_code)]
fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    unsafe {
        let mut head = head;
        let mut front: *const Option<Box<ListNode>> = &head;
        let mut tail: *mut Option<Box<ListNode>> = &mut head;
        for _ in 0..n {
            front = &(*front).as_ref().unwrap().next;
        }
        if (*front).is_none() {
            return head.take().unwrap().next;
        }
        loop {
            front = &(*front).as_ref().unwrap().next;
            if (*front).is_none() {
                break;
            }
            tail = &mut (*tail).as_mut().unwrap().next;
        }
        let next = &mut (*tail).as_mut().unwrap().next;
        *next = next.as_mut().unwrap().next.take();
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils::to_list;

    #[test]
    fn test_19() {
        assert_eq!(remove_nth_from_end(to_list(&[1,2,3,4,5]), 2), to_list(&[1,2,3,5]));
    }
}