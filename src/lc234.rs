use super::utils::ListNode;

#[allow(dead_code)]
fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    if head == None {
        return true;
    }
    let mut v = Vec::new();
    let mut curr = head;
    loop {
        if let Some(node) = curr {
            v.push(node.val);
            curr = node.next;
        } else {
            break;
        }
    }
    let (mut lo, mut hi) = (0, v.len() - 1);
    while lo < hi - 1 {
        if v[lo] != v[hi] {
            return false;
        }
        lo += 1;
        hi -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils::to_linked_list;

    #[test]
    fn test_234() {
        assert!(is_palindrome(to_linked_list(&[1,2,3,3,2,1])));
        assert!(is_palindrome(to_linked_list(&[1,2,3,2,1])));
        assert!(!is_palindrome(to_linked_list(&[1,2,3,2])));
    }
}