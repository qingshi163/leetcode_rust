#[macro_export]
macro_rules! vec_string {
    ($($e:expr),*) => {vec![$($e.to_string()), *]};
    ($($e:expr,)*) => {vec![$($e.to_string()), *]};
}
#[macro_export]
macro_rules! vec_sort {
    ($($e:expr),*) => {{
        let mut tmp = vec![$($e),*];
        tmp.sort();
        tmp
    }};
    ($($e:expr,)*) => {{
        let mut tmp = vec![$($e),*];
        tmp.sort();
        tmp
    }};
}

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
pub fn to_linked_list(v: &[i32]) -> Option<Box<ListNode>> {
    let mut curr = None;
    for &val in v.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = curr;
        curr = Some(Box::new(node));
    }
    curr
}