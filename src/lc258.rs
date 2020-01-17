
#[allow(dead_code)]
fn add_digits(num: i32) -> i32 {
    let mut num = num;
    loop {
        if num < 10 {
            return num;
        }
        let mut n = 0;
        while num != 0 {
            n += num % 10;
            num /= 10;
        }
        num = n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_258() {
        assert_eq!(add_digits(38), 2);
    }
}