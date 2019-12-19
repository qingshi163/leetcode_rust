

#[allow(dead_code)]
fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let mut state = '\0';
    for c in s.chars() {
        match c {
            'I' => { result += 1 },
            'V' => { result += if state=='I' {3} else {5}},
            'X' => { result += if state=='I' {8} else {10}},
            'L' => { result += if state=='X' {30} else {50}},
            'C' => { result += if state=='X' {80} else {100}},
            'D' => { result += if state=='C' {300} else {500}},
            'M' => { result += if state=='C' {800} else {1000}},
            _ => {}
        }
        state = c;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        assert_eq!(roman_to_int("III".to_owned()), 3);
        assert_eq!(roman_to_int("IV".to_owned()), 4);
        assert_eq!(roman_to_int("IX".to_owned()), 9);
        assert_eq!(roman_to_int("LVIII".to_owned()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_owned()), 1994);
    }
}
