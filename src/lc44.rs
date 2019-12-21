
fn __is_match(s: &[char], p: &[char]) -> bool {
    let mut i = 0;
    while i < std::cmp::min(s.len(), p.len()) {
        if s[i] != p[i] && p[i] != '?' {
            break;
        }
        i += 1;
    }
    if i == p.len() {
        return i == s.len();
    }
    if p[i] != '*' {
        return false;
    }
    let mut p_idx = i;
    while p_idx+1 != p.len() && p[p_idx+1] == '*' { p_idx += 1; }
    (i != s.len() && _is_match(&s[i+1..], &p[p_idx..])) ||
    _is_match(&s[i..], &p[p_idx+1..])
}

fn _is_match(s: &[char], p: &[char]) -> bool {
    let (mut s_idx, mut p_idx, mut matched, mut has_star, mut star_idx) = (0, 0, 0, false, 0);
    while s_idx < s.len() {
        if p_idx < p.len() {
            if p[p_idx] == '?' || p[p_idx] == s[s_idx] {
                s_idx += 1;
                p_idx += 1;
                continue;
            }
            if p[p_idx] == '*' {
                has_star = true;
                star_idx = p_idx;
                matched = s_idx;
                p_idx += 1;
                continue;
            }
        }
        if has_star {
            p_idx = star_idx + 1;
            matched += 1;
            s_idx = matched;
            continue;
        }
        return false;
    }
    while p_idx < p.len() && p[p_idx] == '*' { p_idx += 1; }
    p_idx == p.len()
}

#[allow(dead_code)]
fn is_match(s: String, p: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    _is_match(&s, &p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_44() {
        assert!(is_match("abc".to_owned(), "abc".to_owned()));
        assert!(is_match("abc".to_owned(), "???".to_owned()));
        assert!(!is_match("abc".to_owned(), "????".to_owned()));
        assert!(is_match("abc".to_owned(), "???*".to_owned()));
        assert!(is_match("abc".to_owned(), "*".to_owned()));
        assert!(!is_match("abc".to_owned(), "*d".to_owned()));
        assert!(!is_match("bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab".to_owned(),
            "b*b*ab*ba*b**b***bba".to_owned()));
        assert!(is_match("bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab".to_owned(),
            "b*b*ab*ba*b**b***aab".to_owned()));
        assert!(!is_match("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb".to_owned(),
        "**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb".to_owned()));
    }
    //"bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab"
    //"b*b*ab**ba*b**b***bba"
    //"abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb"
    //"**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb"
}