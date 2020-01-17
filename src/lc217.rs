
use std::collections::HashSet;
#[allow(dead_code)]
fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for n in nums {
        if !set.insert(n) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_217() {
        assert!(contains_duplicate(vec![1,2,3,1]));
        assert!(!contains_duplicate(vec![1,2,3,4]));
    }
}