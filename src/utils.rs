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
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[allow(dead_code)]
pub fn to_list(v: &[i32]) -> Option<Box<ListNode>> {
    let mut curr = None;
    for &val in v.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = curr;
        curr = Some(Box::new(node));
    }
    curr
}

use std::rc::Rc;
use std::cell::RefCell;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

#[allow(dead_code)]
pub fn to_tree(v: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if v.is_empty() { return None; }
    let head = Some(Rc::new(RefCell::new(TreeNode::new(v[0].unwrap()))));
    let mut workflow = vec![Rc::clone(head.as_ref().unwrap())];
    let mut newnodes = Vec::new();
    let mut v_idx = 1;
    loop {
        for node in &workflow {
            if v_idx == v.len() { return head; }
            if let Some(val) = v[v_idx] {
                let tmp = Rc::new(RefCell::new(TreeNode::new(val)));
                newnodes.push(Rc::clone(&tmp));
                node.borrow_mut().left = Some(tmp);
            }
            v_idx += 1;
            if v_idx == v.len() { return head; }
            if let Some(val) = v[v_idx] {
                let tmp = Rc::new(RefCell::new(TreeNode::new(val)));
                newnodes.push(Rc::clone(&tmp));
                node.borrow_mut().right = Some(tmp);
            }
            v_idx += 1;
        }
        if newnodes.is_empty() {
            return head;
        } else {
            std::mem::swap(&mut workflow, &mut newnodes);
        }
    }
}


#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(&vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}