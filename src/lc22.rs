
#[allow(dead_code)]
fn generate_parenthesis(n: i32) -> Vec<String> {
    let n = n as usize * 2;
    if n == 0 {
        return Vec::new();
    }
    let mut v = vec![(vec![b'('], 1)];
    for _ in 1..n {
        let mut v2 = Vec::new();
        for pair in &v {
            if pair.1 < n - pair.0.len() {
                let mut s = pair.0.clone();
                s.push(b'(');
                v2.push((s, pair.1 + 1));
            }
            if pair.1 != 0 {
                let mut s = pair.0.clone();
                s.push(b')');
                v2.push((s, pair.1 - 1));
            }
        }
        v = v2;
    }
    let mut result = Vec::new();
    for pair in v {
        unsafe { result.push(String::from_utf8_unchecked(pair.0)); }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        let mut v = generate_parenthesis(3);
        let mut v2 = vec![
            "((()))",
            "(()())",
            "(())()",
            "()(())",
            "()()()"
        ];
        v.sort();
        v2.sort();
        assert_eq!(v, v2);
    }
}