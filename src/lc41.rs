/*
41. First Missing Positive
Hard

Given an unsorted integer array, find the smallest missing positive integer.

Example 1:

Input: [1,2,0]
Output: 3

Example 2:

Input: [3,4,-1,1]
Output: 2

Example 3:

Input: [7,8,9,11,12]
Output: 1

Follow up:

Your algorithm should run in O(n) time and uses constant extra space.

*/

#[allow(dead_code)]
fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut v: Vec<bool> = vec![false; nums.len()];
    for n in nums {
        if n <= 0 || n > v.len() as i32 {
            continue;
        }
        v[(n - 1) as usize] = true;
    }
    let mut ret = 1;
    for b in v {
        if b {
            ret += 1;
        } else {
            break;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(first_missing_positive(vec![1, 2, 3]), 4);
    }
}
