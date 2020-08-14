/* You are given a string, s, and a list of words, words, that are all of the same length. Find all starting indices of substring(s) in s that is a concatenation of each word in words exactly once and without any intervening characters.



Example 1:

Input:
  s = "barfoothefoobarman",
  words = ["foo","bar"]
Output: [0,9]
Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar" respectively.
The output order does not matter, returning [9,0] is fine too.

Example 2:

Input:
  s = "wordgoodgoodgoodbestword",
  words = ["word","good","best","word"]
Output: []
 */

use std::collections::HashMap;
#[allow(dead_code)]
fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let s = s.into_bytes();
    if words.is_empty() {
        return vec![];
    };
    if words[0].is_empty() {
        return (0..s.len() as i32 + 1).collect();
    };
    let section_len = words.len();
    let word_len = words[0].len();
    let cmp_len = section_len * word_len;
    if cmp_len > s.len() {
        return vec![];
    };
    let mut result: Vec<i32> = vec![];
    // Map the str to index of counter
    let mut sets: HashMap<Vec<u8>, usize> = HashMap::with_capacity(words.len());
    // index 0 is for str that not included by words
    let mut base_counter: Vec<i32> = vec![0];
    for word in words {
        sets.entry(word.into_bytes())
            .and_modify(|x| base_counter[*x] += 1)
            .or_insert_with(|| {
                base_counter.push(1);
                base_counter.len() - 1
            });
    }
    for shift in 0..word_len {
        let mut counter = base_counter.clone();
        for (e, i) in (shift..s.len() - word_len + 1).step_by(word_len).enumerate() {
            counter[sets.get(&s[i..i + word_len]).cloned().unwrap_or(0)] -= 1;
            if e + 1 < section_len {
                continue;
            }
            if e >= section_len {
                counter[sets
                    .get(&s[i - cmp_len..i - cmp_len + word_len])
                    .cloned()
                    .unwrap_or(0)] += 1;
            }
            if counter.iter().all(|&x| x == 0) {
                result.push((i - (cmp_len - word_len)) as i32);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_30() {
        assert_eq!(
            find_substring("barfoothefoobarman".into(), vec_string!["foo", "bar"]),
            vec![0, 9]
        );
        assert_eq!(
            find_substring(
                "wordgoodgoodgoodbestword".into(),
                vec_string!["word", "good", "best", "word"]
            ),
            vec![]
        );
        assert_eq!(
            find_substring(
                "wordgoodgoodgoodbestword".into(),
                vec_string!["word","good","best","good"]
            ),
            vec![8]
        );
        assert_eq!(
            find_substring(
                "abcdefg".into(),
                vec_string!["c","d","e","f"]
            ),
            vec![2]
        );
        assert_eq!(find_substring(" ".into(), vec_string!["", ""]), vec![0, 1]);
        assert_eq!(find_substring("".into(), vec_string!["", ""]), vec![0]);
    }
}
