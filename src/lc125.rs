
#[allow(dead_code)]
fn is_palindrome(s: String) -> bool {
    if s.is_empty() {
        return true;
    }
    let s: Vec<char> = s.chars().collect();
    let (mut left, mut right) = (0, s.len() - 1);
    while left < right + 1 {
        if !s[left].is_alphanumeric() { left += 1; continue }
        if !s[right].is_alphanumeric() { right -= 1; continue }
        if !s[left].eq_ignore_ascii_case(&s[right]) {
            return false;
        }
        left += 1; right -= 1;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_125() {
        assert!(!is_palindrome("race a car".to_owned()));
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_owned()));
    }
}