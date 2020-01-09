

use std::rc::Rc;
use std::cell::RefCell;
use super::utils::TreeNode;
#[allow(dead_code)]
fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    p == q
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::to_tree;

    #[test]
    fn test_100() {
        assert!(is_same_tree(None, None));
        assert!(is_same_tree(tree![1,2,3], tree![1,2,3]));
        assert!(is_same_tree(tree![1,2,3,null,4], tree![1,2,3,null,4]));
    }
}