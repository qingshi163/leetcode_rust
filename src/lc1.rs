
use std::collections::HashMap;
#[allow(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.into_iter().enumerate() {
        match map.get(&(target - num)) {
            Some(val) => return vec![*val, i as i32],
            None => { map.insert(num, i as i32); }
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0,1]);
    }
}