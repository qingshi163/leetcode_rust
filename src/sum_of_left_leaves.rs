/*
Sum of Left Leaves

Find the sum of all left leaves in a given binary tree.

Example:

    3
   / \
  9  20
    /  \
   15   7

There are two left leaves in the binary tree, with values 9 and 15 respectively. Return 24.

 */

use super::utils::*;
use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(root) => dfs(root, false),
        None => 0,
    }
}

fn dfs(node: Rc<RefCell<TreeNode>>, is_left_node: bool) -> i32 {
    let mut ret = 0;
    let mut is_leaf = true;
    if let Some(left) = node.borrow().left.clone() {
        ret += dfs(left, true);
        is_leaf = false;
    }
    if let Some(right) = node.borrow().right.clone() {
        ret += dfs(right, false);
        is_leaf = false;
    }
    if is_left_node && is_leaf { node.borrow().val } else { ret }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(sum_of_left_leaves(tree![3, 9, 20, null, null, 15, 7]), 24);
        assert_eq!(sum_of_left_leaves(tree![1, 2, 3, 4, 5]), 4);
    }
}
