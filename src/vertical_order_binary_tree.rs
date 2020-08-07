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
use std::cell::RefCell;
use std::rc::Rc;
struct VerticalOrderBinaryTreeSolution {
    positive_wing: Vec<Vec<(i32, i32)>>,
    negative_wing: Vec<Vec<(i32, i32)>>,
}
impl VerticalOrderBinaryTreeSolution {
    fn dfs(&mut self, node: Rc<RefCell<TreeNode>>, x: i32, y: i32) {
        let (wing, index) = if x >= 0 {
            (&mut self.positive_wing, x as usize)
        } else {
            (&mut self.negative_wing, (x.abs() - 1) as usize)
        };
        if index == wing.len() {
            wing.push(vec![]);
        }
        wing[index].push((y, node.borrow().val));
        if let Some(left) = node.borrow().left.clone() {
            self.dfs(left, x - 1, y + 1);
        }
        if let Some(right) = node.borrow().right.clone() {
            self.dfs(right, x + 1, y + 1);
        }
    }
}
#[allow(dead_code)]
pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let node = match root {
        None => return vec![],
        Some(node) => node,
    };
    let mut solution = VerticalOrderBinaryTreeSolution {
        positive_wing: vec![],
        negative_wing: vec![],
    };
    solution.dfs(node, 0, 0);
    let mut ret = solution.negative_wing;
    ret.reverse();
    ret.append(&mut solution.positive_wing);
    ret.iter_mut().map(|x| {
        x.sort_unstable_by(|a, b| Ord::cmp(a, b));
        x.iter().map(|&pair| pair.1).collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_order_binary_tree() {
        assert_eq!(vertical_traversal(tree![3,9,20,null,null,15,7]), vec![vec![9], vec![3,15], vec![20], vec![7]]);
    }
}
