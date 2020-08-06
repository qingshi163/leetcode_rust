
#[allow(dead_code)]
fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    let mut map: HashSet<i32> = HashSet::with_capacity(nums.len());
    let mut i = 0;
    while i != nums.len() {
        if map.insert(nums[i]) {
            nums.swap_remove(i);
        } else {
            i += 1;
        }
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate_in_array() {
        assert_eq!(find_duplicates(vec![4,3,2,7,8,2,3,1]), vec![2,3]);
    }
}