
#[allow(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    for i in 0..(len-1) {
        for j in (i+1)..len {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return Vec::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lc1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0,1]);
    }
}