/*
949. Largest Time for Given Digits
Easy

Given an array of 4 digits, return the largest 24 hour time that can be made.

The smallest 24 hour time is 00:00, and the largest is 23:59.  Starting from 00:00, a time is larger if more time has elapsed since midnight.

Return the answer as a string of length 5.  If no valid time can be made, return an empty string.



Example 1:

Input: [1,2,3,4]
Output: "23:41"

Example 2:

Input: [5,5,5,5]
Output: ""



Note:

    A.length == 4
    0 <= A[i] <= 9


*/

#[allow(dead_code)]
fn largest_time_from_digits(mut a: Vec<i32>) -> String {
    a.sort_unstable_by(|a, b| Ord::cmp(b, a));
    h1(a).unwrap_or_default()
}

fn h1(a: Vec<i32>) -> Option<String> {
    let mut ret = String::new();
    let pos = a.iter().position(|&x| x <= 2)?;
    for i in pos..a.len() {
        let val = a[i];
        let mut b = a.clone();
        b.remove(i);
        if let Some(s) = h2(b, if val == 2 { 3 } else { 9 }) {
            ret = std::cmp::max(ret, format!("{}{}", val, s));
        }
    }
    if ret.is_empty() {
        None
    } else {
        Some(ret)
    }
}

fn h2(a: Vec<i32>, edge: i32) -> Option<String> {
    let mut ret = String::new();
    let pos = a.iter().position(|&x| x <= edge)?;
    for i in pos..a.len() {
        let val = a[i];
        let mut b = a.clone();
        b.remove(i);
        if let Some(s) = m1(b) {
            ret = std::cmp::max(ret, format!("{}:{}", val, s));
        }
    }
    if ret.is_empty() {
        None
    } else {
        Some(ret)
    }
}

fn m1(mut a: Vec<i32>) -> Option<String> {
    let pos = a.iter().position(|&x| x <= 5)?;
    let val = a[pos];
    a.remove(pos);
    Some(format!("{}{}", val, a[0]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(largest_time_from_digits(vec![1, 2, 3, 4]), "23:41");
        assert_eq!(largest_time_from_digits(vec![5, 5, 5, 5]), "");
        assert_eq!(largest_time_from_digits(vec![1, 9, 6, 0]), "19:06");
    }
}
