
#[allow(dead_code)]
#[allow(non_snake_case)]
fn lengthOfLongestSubstring(s: String) -> i32 {
    let mut sub: Vec<u8> = Vec::new();
    let mut count: i32 = 0;
    for &b in s.as_bytes() {
        if let Some(index) = sub.iter().position(|&a| a==b) {
            count = std::cmp::max(count, sub.len() as i32);
            let (_,_sub) = sub.split_at_mut(index+1);
            sub = _sub.to_vec();
        }
        sub.push(b);
    }
    std::cmp::max(count, sub.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(lengthOfLongestSubstring(String::from("123456789123")), 9);
        assert_eq!(lengthOfLongestSubstring(String::from("")), 0);
        assert_eq!(lengthOfLongestSubstring(String::from("123123")), 3);
        assert_eq!(lengthOfLongestSubstring(String::from("acddddddca")), 3);
        assert_eq!(lengthOfLongestSubstring(String::from("acdefghca")), 7);
        assert_eq!(lengthOfLongestSubstring(String::from("abcdefabcdefg")), 7);
        assert_eq!(lengthOfLongestSubstring(String::from("dvdf")), 3);
    }
}