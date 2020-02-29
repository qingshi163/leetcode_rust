
#[allow(dead_code)]
fn title_to_number(s: String) -> i32 {
    // s.bytes().rev().enumerate().fold(0, |acc, (i, b)| acc + (b - b'A' + 1) as i32 * 26i32.pow(i as u32))
    s.bytes().fold(0, |acc, b| acc * 26 + (b - b'A' + 1) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_171() {
        assert_eq!(title_to_number("ZY".to_owned()), 701);
        assert_eq!(title_to_number("AB".to_owned()), 28);
    }
}