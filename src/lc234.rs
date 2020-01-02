// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  #[allow(dead_code)]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
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

    #[test]
    fn test_234() {
    }
}