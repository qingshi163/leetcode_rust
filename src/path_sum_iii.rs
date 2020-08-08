// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use super::utils::*;
use std::rc::Rc;
use std::cell::RefCell;
struct PathSumIIISolution {
    target: i32,
    result: i32,
}
impl PathSumIIISolution {
    fn dfs(&mut self, node: Rc<RefCell<TreeNode>>) -> Vec<i32> {
        let mut ret = vec![];
        if let Some(left) = node.borrow().left.clone() {
            ret.append(&mut self.dfs(left));
        }
        if let Some(right) = node.borrow().right.clone() {
            ret.append(&mut self.dfs(right));
        }
        ret.push(0);
        ret.iter_mut().for_each(|x| *x += node.borrow().val);
        self.result += ret.iter().filter(|&&x| x == self.target).count() as i32;
        ret
    }
}
#[allow(dead_code)]
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    let root = match root {
        None => return 0,
        Some(node) => node,
    };
    let mut solution = PathSumIIISolution { target: sum, result: 0 };
    solution.dfs(root);
    solution.result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_sum_iii() {
        assert_eq!(path_sum(tree![10,5,-3,3,2,null,11,3,-2,null,1], 8), 3);
    }
}