

#[allow(dead_code)]
fn _is_match(s: &[char], p: &[char]) -> bool {
    if p.is_empty() {
        return s.is_empty();
    }
    let first_match = !s.is_empty() && (s[0] == p[0] || p[0] == '.');
    if p.len() >= 2 && p[1] == '*' {
        _is_match(s, &p[2..]) ||
        (first_match && _is_match(&s[1..], p))
    } else {
        first_match && _is_match(&s[1..], &p[1..])
    }
}

#[allow(dead_code)]
fn is_match(s: String, p: String) -> bool {
    let s_chars = s.chars().collect::<Vec<char>>();
    let p_chars = p.chars().collect::<Vec<char>>();
    _is_match(&s_chars, &p_chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        assert!(is_match("abc".to_owned(), "abc".to_owned()));
        assert!(!is_match("a".to_owned(), "ab".to_owned()));
        assert!(is_match("ab".to_owned(), "..".to_owned()));
        assert!(is_match("ab".to_owned(), ".*".to_owned()));
    }
}
