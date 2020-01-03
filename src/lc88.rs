
#[allow(dead_code)]
fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m as usize;
    let mut n = n as usize;
    for i in (0..m+n).rev() {
        nums1[i] = {
            if m == 0 { n -= 1; nums2[n] }
            else if n == 0 { m -= 1; nums1[m] }
            else if nums1[m-1] > nums2[n-1] { m -= 1; nums1[m] }
            else {n -= 1; nums2[n] }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88() {
        let mut nums = vec![1,2,3,0,0,0];
        merge(&mut nums, 3, &mut vec![2,5,6], 3);
        assert_eq!(nums, [1,2,2,3,5,6]);
    }
}