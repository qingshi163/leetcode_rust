#[allow(dead_code)]
fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = i32::min_value();
    let mut adding = 0;
    for i in 0..nums.len() {
        adding = if adding <= 0 {
            nums[i]
        } else {
            nums[i] + adding
        };
        if adding > max {
            max = adding;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53() {
        assert_eq!(max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    }
}