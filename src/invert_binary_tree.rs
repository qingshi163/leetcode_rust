
use std::rc::Rc;
use std::cell::RefCell;
use super::utils::*;
#[allow(dead_code)]
fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_some() {
        let mut l = &root.as_ref().unwrap().borrow().left;
        let mut r = &root.as_ref().unwrap().borrow().right;
        std::mem::swap(&mut l, &mut r);
    }
    root
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_invert_binary_tree() {
        // assert_eq!(invert_tree(tree![4,2,7,1,3,6,9]), tree![4,7,2,9,6,3,1]);
    }
}