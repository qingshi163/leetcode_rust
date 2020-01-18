
use std::collections::HashMap;
#[allow(dead_code)]
fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        if let Some(old) = map.insert(nums[i], i) {
            if i - old <= k as usize {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_219() {
        assert!(contains_nearby_duplicate(vec![1,2,3,1], 3));
        assert!(contains_nearby_duplicate(vec![1,0,1,1], 1));
        assert!(!contains_nearby_duplicate(vec![1,2,3,1,2,3], 2));
    }
}