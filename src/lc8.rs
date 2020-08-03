
#[allow(dead_code)]
fn my_atoi(str: String) -> i32 {
    let mut result = 0i64;
    let mut it = str.chars().skip_while(|&c| c == ' ').peekable();
    let negative = match it.peek() {
        Some(c) => match c {
            '-' => {
                it.next();
                true
            },
            '+' => {
                it.next();
                false
            }
            _ => { false }
        },
        None => { return 0; }
    };
    it.take_while(|c| c.is_digit(10)).skip_while(|&c| c=='0').take(11).for_each(|c| {
        result *= 10;
        result += c.to_digit(10).unwrap() as i64;
    });
    if negative {
        if -result >= std::i32::MIN.into() {-result as i32} else {std::i32::MIN}
    } else if result <= std::i32::MAX.into() { result as i32 } else {std::i32::MAX}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(my_atoi("123".to_owned()), 123);
        assert_eq!(my_atoi("0321".to_owned()), 321);
        assert_eq!(my_atoi("  321".to_owned()), 321);
        assert_eq!(my_atoi("   0123".to_owned()), 123);
        assert_eq!(my_atoi("".to_owned()), 0);
        assert_eq!(my_atoi("  0000123abc".to_owned()), 123);
        assert_eq!(my_atoi("  -0000123abc".to_owned()), -123);
        assert_eq!(my_atoi("-123abc".to_owned()), -123);
        assert_eq!(my_atoi("   -99283122322abc".to_owned()), std::i32::MIN);
        assert_eq!(my_atoi("   99283122322abc".to_owned()), std::i32::MAX);
        assert_eq!(my_atoi(std::i32::MIN.to_string()), std::i32::MIN);
        assert_eq!(my_atoi(std::i32::MAX.to_string()), std::i32::MAX);
        assert_eq!(my_atoi("9223372036854775808".to_owned()), std::i32::MAX);
        assert_eq!(my_atoi("-9223372036854775808".to_owned()), std::i32::MIN);
        assert_eq!(my_atoi("20000000000000000000".to_owned()), std::i32::MAX);
        assert_eq!(my_atoi("  0000000000012345678".to_owned()), 12345678);
    }
}