
use std::rc::Rc;
use std::cell::RefCell;
use super::utils::TreeNode;
#[allow(dead_code)]
fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(root) => helper(&root, 1)
    }
}
fn helper(root: &Rc<RefCell<TreeNode>>, level: i32) -> i32 {
    match (&root.borrow().left, &root.borrow().right) {
        (None, None) => level,
        (None, Some(node)) | (Some(node), None) => {
            helper(&node, level + 1)
        },
        (Some(left), Some(right)) => {
            std::cmp::min(helper(&left, level + 1), helper(&right, level + 1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::to_tree;

    #[test]
    fn test_111() {
        assert_eq!(min_depth(tree![3,9,20,null,null,15,7]), 2);
    }
}