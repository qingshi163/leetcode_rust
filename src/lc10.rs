

#[allow(dead_code)]
fn is_match(s: String, p: String) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        is_match("abc".to_owned(), "abc".to_owned());
    }
}
