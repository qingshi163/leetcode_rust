use std::rc::Rc;
use std::cell::RefCell;
use super::utils::*;
#[allow(dead_code)]
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() { return vec![]; }
    use std::collections::VecDeque;
    let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    q.push_back(root.unwrap());
    let mut result = vec![];
    let mut level = 1;
    while !q.is_empty() {
        let k = q.len();
        let mut row: Vec<i32> = vec![];
        for _ in 0..k {
            let node = q.pop_front().unwrap();
            row.push(node.borrow().val);
            if let Some(l) = &node.clone().borrow().left {
                q.push_back(l.clone());
            }
            if let Some(r) = &node.clone().borrow().right {
                q.push_back(r.clone());
            }
        }
        if level % 2 == 0 {
            row.reverse();
        }
        level += 1;
        result.push(row);
    }
    result
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_binary_tree_zigzag() {
    }
}