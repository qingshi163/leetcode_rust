

#[allow(dead_code)]
fn next_permutation(nums: &mut Vec<i32>) {
    if nums.is_empty() {
        return;
    }
    let mut i = nums.len() - 1;
    while i > 0 && nums[i-1] >= nums[i] {
        i -= 1;
    }
    if i != 0 {
        let mut j = nums.len() - 1;
        while j > 0 && nums[j] <= nums[i-1] {
            j -= 1;
        }
        nums.swap(i-1, j);
    }
    nums[i..].reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {
        let mut v = vec![1,2,3];
        next_permutation(&mut v);
        assert_eq!(v, vec![1,3,2]);
        let mut v = vec![3,2,1];
        next_permutation(&mut v);
        assert_eq!(v, vec![1,2,3]);
        let mut v = vec![1,1,5];
        next_permutation(&mut v);
        assert_eq!(v, vec![1,5,1]);
        let mut v = vec![1,3,2];
        next_permutation(&mut v);
        assert_eq!(v, vec![2,1,3]);
    }
}
