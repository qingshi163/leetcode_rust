#[allow(dead_code)]
fn letter_combinations(digits: String) -> Vec<String> {
    let map = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut result: Vec<String> = Vec::new();
    for b in digits.bytes() {
        if b < b'2' || b > b'9' {
            return result;
        }
        let b = (b - b'2') as usize;
        if result.is_empty() {
            result = map[b].chars().map(|c| c.to_string()).collect();
            continue;
        }
        let mut v = Vec::new();
        for c in map[b].chars() {
            for x in result.iter() {
                let mut tmp = x.clone();
                tmp.push(c);
                v.push(tmp);
            }
        }
        result = v;
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(letter_combinations("2".to_owned()), ["a", "b", "c"]);
        assert_eq!(
            letter_combinations("23".to_owned()),
            ["ad", "bd", "cd", "ae", "be", "ce", "af", "bf", "cf"]
        );
    }
}
