#[allow(dead_code)]
fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut result: i64 = 0;
    while x != 0 {
        result *= 10;
        result += x as i64 % 10;
        x /= 10;
    }
    if result > std::i32::MAX.into() || result < std::i32::MIN.into() {
        0
    } else {
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(-2147483648), 0);
    }
}
