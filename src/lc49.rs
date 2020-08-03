
use std::collections::HashMap;
#[allow(dead_code)]
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::<Vec<u8>,Vec<String>>::new();
    for s in &strs {
        let mut key = s.bytes().collect::<Vec<u8>>();
        key.sort_unstable();
        map.entry(key).and_modify(|val| val.push(s.clone())).or_insert_with(|| vec![s.clone()]);
    }
    map.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_49() {
        let strs = vec_string!["eat","tea","tan","ate","nat","bat",];
        let mut result = group_anagrams(strs);
        result.iter_mut().for_each(|r| r.sort());
        result.sort();
        assert_eq!(result,
            vec_sort![vec_sort!["ate","eat","tea"],vec_sort!["nat","tan"],vec_sort!["bat"]]);
    }
}