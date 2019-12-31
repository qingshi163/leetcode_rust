
#[allow(dead_code)]
fn length_of_last_word(s: String) -> i32 {
    s.as_bytes().iter().rev().skip_while(|&&x| x==b' ').take_while(|&&x| x!=b' ').count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_58() {
        assert_eq!(length_of_last_word("HELLO WORLD  ".to_owned()), 5);
    }
}