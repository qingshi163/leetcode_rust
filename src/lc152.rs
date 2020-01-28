
#[allow(dead_code)]
fn max_product(nums: Vec<i32>) -> i32 {
    let mut max = i32::min_value();
    for i in 0..nums.len() {
        nums[i..].iter().fold(1, |acc, x| {
            let v = x * acc;
            max = std::cmp::max(v, max);
            v
        });
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_152() {
        assert_eq!(max_product(vec![2,3,-2,4]), 6);
    }
}