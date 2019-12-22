

#[allow(dead_code)]
fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![-1, -1];
    if nums.is_empty() {
        return result;
    }
    let (mut start, mut end) = (0, nums.len() - 1);
    while start < end {
        let mid = start + (end - start) / 2;
        if nums[mid] < target {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    if nums[start] != target {
        return result;
    }
    result[0] = start as i32;

    end = nums.len() - 1;
    while start < end {
        let mid = start + (end - start) / 2 + 1;
        if nums[mid] > target {
            end = mid - 1;
        } else {
            start = mid;
        }
    }
    result[1] = end as i32;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {
        assert_eq!(search_range(vec![1,3,3,3,8], 3), [1, 3]);
    }
}