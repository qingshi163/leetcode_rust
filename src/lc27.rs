

#[allow(dead_code)]
fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    while i != nums.len() {
        if nums[i] == val {
            nums.swap_remove(i);
        } else {
            i += 1;
        }
    }
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27() {
        let mut v = vec![3,2,2,3];
        assert_eq!(remove_element(&mut v, 2), 2);
        assert_eq!(v, vec![3,3]);
    }
}
