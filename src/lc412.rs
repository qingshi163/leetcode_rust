
#[allow(dead_code)]
fn fizz_buzz(n: i32) -> Vec<String> {
    let mut next3 = 3;
    let mut next5 = 5;
    let mut ret = Vec::with_capacity(n as usize);
    for num in 1..=n {
        let mut s = String::new();
        if num == next3 {
            s.push_str("Fizz");
            next3 += 3;
        }
        if num == next5 {
            s.push_str("Buzz");
            next5 += 5;
        }
        if s.is_empty() {
            s.push_str(&num.to_string());
        }
        ret.push(s);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(fizz_buzz(15), vec_string![
            "1",
            "2",
            "Fizz",
            "4",
            "Buzz",
            "Fizz",
            "7",
            "8",
            "Fizz",
            "Buzz",
            "11",
            "Fizz",
            "13",
            "14",
            "FizzBuzz"
        ]);
    }
}