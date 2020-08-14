

#[allow(dead_code)]
fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() { return 0; }
    if haystack.len() < needle.len() { return -1; }
    let haystack = haystack.into_bytes();
    let needle = needle.into_bytes();
    for i in 0..=haystack.len() - needle.len() {
        if needle == &haystack[i..i + needle.len()] {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_28() {
        assert_eq!(str_str("hello".to_owned(), "ll".to_owned()), 2);
        assert_eq!(str_str("a".to_owned(), "a".to_owned()), 0);
        assert_eq!(str_str("".to_owned(), "a".to_owned()), -1);
    }
}
