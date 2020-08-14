

#[allow(dead_code)]
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut last = 0;
    for i in 1..nums.len() {
        if nums[i] == nums[i-1] {
            continue;
        }
        last += 1;
        nums[last] = nums[i];
    }
    last as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26() {
        let mut v = vec![1,1,2,2,3];
        assert_eq!(remove_duplicates(&mut v), 3);
    }
}
