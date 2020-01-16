
#[allow(dead_code)]
fn convert_to_title(n: i32) -> String {
    let mut n = n;
    let mut s: Vec<u8> = Vec::new();
    while n != 0 {
        let b = (n % 26) as u8;
        if b != 0 {
            s.push(b'A' + b - 1);
        } else {
            s.push(b'Z');
            n -= 26;
        }
        n /= 26;
    }
    unsafe { String::from_utf8_unchecked(s.iter().rev().cloned().collect()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_168() {
        assert_eq!(convert_to_title(1), "A");
        assert_eq!(convert_to_title(26), "Z");
        assert_eq!(convert_to_title(701), "ZY");
    }
}