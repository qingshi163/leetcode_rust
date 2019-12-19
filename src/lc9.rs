
#[allow(dead_code)]
fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) { return false; }
    let mut x = x;
    let mut y = 0;
    while x > y {
        y *= 10;
        y += x % 10;
        x /= 10;
    }
    x == y || x == y / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert!(is_palindrome(12321));
        assert!(is_palindrome(0));
        assert!(is_palindrome(1));
        assert!(is_palindrome(121));
        assert!(!is_palindrome(12));
        assert!(!is_palindrome(1121));
        assert!(is_palindrome(1221));
        assert!(is_palindrome(1111));
        assert!(!is_palindrome(10));
    }
}