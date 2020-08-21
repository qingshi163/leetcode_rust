/*
Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it is able to trap after raining.


The above elevation map is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped. Thanks Marcos for contributing this image!

Example:

Input: [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6
 */

#[allow(dead_code)]
fn trap(height: Vec<i32>) -> i32 {
    use std::cmp::max;
    if height.is_empty() {
        return 0;
    }
    let (mut left, mut right) = (0, height.len() - 1);
    let (mut left_height, mut right_height) = (0, 0);
    let mut ret = 0;
    while left < right {
        if height[left] < height[right] {
            left_height = max(left_height, height[left]);
            ret += max(0, left_height - height[left]);
            left += 1;
        } else {
            right_height = max(right_height, height[right]);
            ret += max(0, right_height - height[right]);
            right -= 1;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(trap(vec![5,1,0,2,1,0,1,3,2,1,2,1]), 14);
    }
}
