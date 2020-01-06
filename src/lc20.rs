#[allow(dead_code)]
fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if !stack.is_empty() && stack[stack.len() - 1] == '(' {
                    stack.pop();
                } else {
                    return false;
                }
            },
            '}' => {
                if !stack.is_empty() && stack[stack.len() - 1] == '{' {
                    stack.pop();
                } else {
                    return false;
                }
            },
            ']' => {
                if !stack.is_empty() && stack[stack.len() - 1] == '[' {
                    stack.pop();
                } else {
                    return false;
                }
            },
            _ => return false
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert!(is_valid("()[]{}".to_owned()));
        assert!(!is_valid("()]{}".to_owned()));
        assert!(is_valid("({[]})".to_owned()));
    }
}
