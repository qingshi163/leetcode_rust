
use std::rc::Rc;
use std::cell::RefCell;
use super::utils::TreeNode;
#[allow(dead_code)]
fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    helper(&nums)
}
fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    match nums.len() {
        0 => None,
        1 => Some(Rc::new(RefCell::new(TreeNode::new(nums[0])))),
        len => {
            let mid = len / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val:   nums[mid],
                left:  helper(&nums[0..mid]),
                right: helper(&nums[mid+1..len])
            })))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_108() {
    }
}