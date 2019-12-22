

#[allow(dead_code)]
fn jump(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }
    let mut start = 1;
    let mut end = nums[0] as usize;
    let mut cover = end;
    let mut count = 1;
    // println!("start: {}, end: {}", start, end);
    while cover < nums.len() - 1 {
        for i in start..end+1 {
            cover = std::cmp::max(cover, i + nums[i] as usize);
        }
        count += 1;
        start = end + 1;
        end = cover;
        // println!("cover: {}, count: {}", cover, count);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_45() {
        assert_eq!(jump(vec![2,3,1,1,4]), 2);
        assert_eq!(jump(vec![2,1,1,1,4]), 3);
    }
}