
#[allow(dead_code)]
fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let mut closest = 0;
    let mut delta = i32::max_value();
    for a in 0..nums.len() - 2 {
        let mut b = a + 1;
        let mut c = nums.len() - 1;
        while b < c {
            let sum = nums[a] + nums[b] + nums[c];
            if sum == target {
                return sum;
            }
            if (sum - target).abs() < delta {
                delta = (sum - target).abs();
                closest = sum;
            }
            if (sum - target).abs() < (closest - target).abs() {
                closest = sum;
            }
            if sum < target {
                b += 1;
            } else {
                c -= 1;
            }
        }
    }
    closest
}
    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        assert_eq!(three_sum_closest(vec![-1,2,1,-4], 1), 2);
    }
}
