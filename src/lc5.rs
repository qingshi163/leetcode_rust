
#[allow(dead_code)]
fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return String::from("");
    }
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = 0;
    for i in 0..bytes.len() {
        let len = std::cmp::max(
            expand_from(bytes, i, i),
            expand_from(bytes, i, i+1)
        );
        if len > end - start {
            // 这里有单数和双数的问题。要确保两种状态下的正确
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }
    std::str::from_utf8(&bytes[start..end+1]).unwrap().to_owned()
}

#[allow(dead_code)]
fn expand_from(bytes: &[u8], l: usize, r: usize) -> usize {
    let mut l = l;
    let mut r = r;
    let len = bytes.len();
    while r < len && bytes[l]==bytes[r] {
        if l == 0 { r+=2; break; }
        l-=1;
        r+=1;
    }
    if r-l < 2 {0} else {r-l-1}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lc5() {
        assert_eq!(longest_palindrome(String::from("abcdedcba")), String::from("abcdedcba"));
        assert_eq!(longest_palindrome(String::from("aabcba")), String::from("abcba"));
        assert_eq!(longest_palindrome(String::from("")), String::from(""));
        assert_eq!(longest_palindrome(String::from("adeedaccc")), String::from("adeeda"));
        assert_eq!(longest_palindrome(String::from("a")), String::from("a"));
        assert_eq!(longest_palindrome(String::from("aaabb")), String::from("aaa"));
        assert_eq!(longest_palindrome(String::from("aabbb")), String::from("bbb"));
        assert_eq!(longest_palindrome(String::from("abacdfgdcaba")), String::from("aba"));
    }
}