/*
Given a string which consists of lowercase or uppercase letters, find the length of the longest palindromes that can be built with those letters.

This is case sensitive, for example "Aa" is not considered a palindrome here.

Note:
Assume the length of given string will not exceed 1,010.

Example:

Input:
"abccccdd"

Output:
7

Explanation:
One longest palindrome that can be built is "dccaccd", whose length is 7.
 */

#[allow(dead_code)]
fn longest_palindrome(s: String) -> i32 {
    let mut sets = [0; (b'z' - b'A' + 1) as usize];
    for c in s.bytes() {
        sets[(c - b'A') as usize] += 1;
    }
    let sets: Vec<i32> = sets.iter().filter(|&&x| x != 0).cloned().collect();
    let (even, odd): (Vec<i32>, Vec<i32>) = sets.into_iter().partition(|&x| x % 2 == 0);
    let has_odd = if odd.is_empty() { 0 } else { 1 };
    even.into_iter().sum::<i32>() + odd.into_iter().map(|x| x - 1).sum::<i32>() + has_odd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome("abccccdd".into()), 7);
    }
}
