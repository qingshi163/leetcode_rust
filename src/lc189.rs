
#[allow(dead_code)]
fn rotate(nums: &mut Vec<i32>, k: i32) {
    let v = nums.clone();
    (0..nums.len()).cycle().skip(k as usize).take(nums.len()).enumerate().for_each(|(i, j)| {
        nums[j] = v[i];
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_189() {
        let mut v = vec![1,2,3,4,5,6,7];
        rotate(&mut v, 3);
        assert_eq!(v, [5,6,7,1,2,3,4]);
    }
}