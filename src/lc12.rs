
#[allow(dead_code)]
fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut result = String::new();
    while num >= 1000 {
        result.push('M');
        num -= 1000;
    }
    if num >= 900 {
        result.push_str("CM");
        num -= 900;
    }
    if num >= 500 {
        result.push('D');
        num -= 500;
    }
    if num >= 400 {
        result.push_str("CD");
        num -= 400;
    }
    while num >= 100 {
        result.push('C');
        num -= 100;
    }
    if num >= 90 {
        result.push_str("XC");
        num -= 90;
    }
    if num >= 50 {
        result.push('L');
        num -= 50;
    }
    if num >= 40 {
        result.push_str("XL");
        num -= 40;
    }
    while num >= 10 {
        result.push('X');
        num -= 10;
    }
    if num >= 9 {
        result.push_str("IX");
        num -= 9;
    }
    if num >= 5 {
        result.push('V');
        num -= 5;
    }
    if num >= 4 {
        result.push_str("IV");
        num -= 4;
    }
    while num > 0 {
        result.push('I');
        num -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(int_to_roman(3), "III");
        assert_eq!(int_to_roman(4), "IV");
        assert_eq!(int_to_roman(9), "IX");
        assert_eq!(int_to_roman(19), "XIX");
        assert_eq!(int_to_roman(58), "LVIII");
        assert_eq!(int_to_roman(1994), "MCMXCIV");
    }
}