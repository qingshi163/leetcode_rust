use std::cmp::Ordering::*;

#[allow(dead_code)]
fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    if nums.len() < 3 {
        return results;
    }
    nums.sort_unstable();
    for a in 0..nums.len() - 2 {
        if a != 0 && nums[a] == nums[a - 1] {
            continue;
        }
        let mut b = a + 1;
        let mut c = nums.len() - 1;
        while b < c {
            match (nums[a] + nums[b] + nums[c]).cmp(&0) {
                Less => b += 1,
                Greater => c -= 1,
                Equal => {
                    results.push(vec![nums[a], nums[b], nums[c]]);
                    while b < c && nums[b] == nums[b + 1] {
                        b += 1;
                    }
                    while b < c && nums[c] == nums[c - 1] {
                        c -= 1;
                    }
                    b += 1;
                    c -= 1;
                }
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            [[-1, -1, 2], [-1, 0, 1]]
        );
        assert_eq!(three_sum(vec![-2, 0, 1, 1, 2]), [[-2, 0, 2], [-2, 1, 1]]);
    }
}
