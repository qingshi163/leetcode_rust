
use std::rc::Rc;
use std::cell::RefCell;
use super::utils::TreeNode;
#[allow(dead_code)]
fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    check_height(&root).is_some()
}
fn check_height(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    match root {
        None => Some(0),
        Some(node) => {
            let l = check_height(&node.borrow().left)?;
            let r = check_height(&node.borrow().right)?;
            if (l-r).abs() < 2 { Some(1 + std::cmp::max(l, r)) } else { None }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::to_tree;

    #[test]
    fn test_110() {
        assert!(is_balanced(tree![3,9,20,null,null,15,7]));
        assert!(!is_balanced(tree![1,2,2,3,3,null,null,4,4]));
    }
}