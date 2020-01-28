
#[allow(dead_code)]
fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_151() {
        assert_eq!(reverse_words("the sky is blue!".to_owned()), "blue! is sky the");
    }
}