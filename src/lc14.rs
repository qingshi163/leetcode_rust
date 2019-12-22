
#[allow(dead_code)]
fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = String::new();
    if strs.is_empty() {
        return result;
    }
    let mut iter_vec: Vec<std::str::Chars> = strs.iter().map(|s| s.chars()).collect();
    loop {
        let mut iters = iter_vec.iter_mut();
        let curr = iters.next().unwrap().next();
        if curr.is_some() && iters.all(|it| it.next() == curr) {
            result.push(curr.unwrap());
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        let input = vec!["abcd", "abc", "abcddd", "ab"];
        let input: Vec<String> = input.iter().map(|s| s.to_string()).collect();
        assert_eq!(longest_common_prefix(input), "ab");
        let input = vec!["", "", "acc", ""];
        let input: Vec<String> = input.iter().map(|s| s.to_string()).collect();
        assert_eq!(longest_common_prefix(input), "");
    }
}
