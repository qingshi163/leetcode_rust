
#[allow(dead_code)]
fn max_area(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut max = 0;
    while left < right {
        max = std::cmp::max(max,
            std::cmp::min(height[left], height[right]) * (right - left) as i32);
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}