#[allow(dead_code)]
fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = i32::min_value();
    let mut adding = 0;
    for n in nums {
        adding = if adding <= 0 {
            n
        } else {
            n + adding
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