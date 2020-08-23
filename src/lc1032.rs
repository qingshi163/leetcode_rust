/*
1032. Stream of Characters
Hard

Implement the StreamChecker class as follows:

    StreamChecker(words): Constructor, init the data structure with the given words.
    query(letter): returns true if and only if for some k >= 1, the last k characters queried (in order from oldest to newest, including this letter just queried) spell one of the words in the given list.



Example:

StreamChecker streamChecker = new StreamChecker(["cd","f","kl"]); // init the dictionary.
streamChecker.query('a');          // return false
streamChecker.query('b');          // return false
streamChecker.query('c');          // return false
streamChecker.query('d');          // return true, because 'cd' is in the wordlist
streamChecker.query('e');          // return false
streamChecker.query('f');          // return true, because 'f' is in the wordlist
streamChecker.query('g');          // return false
streamChecker.query('h');          // return false
streamChecker.query('i');          // return false
streamChecker.query('j');          // return false
streamChecker.query('k');          // return false
streamChecker.query('l');          // return true, because 'kl' is in the wordlist



Note:

    1 <= words.length <= 2000
    1 <= words[i].length <= 2000
    Words will only consist of lowercase English letters.
    Queries will only consist of lowercase English letters.
    The number of queries is at most 40000.

*/

#[derive(Debug, Default)]
struct Trie {
    next: [Option<Box<Trie>>; 26],
    is_word: bool,
}

struct StreamChecker {
    head: Trie,
    queue: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut head = Trie::default();
        for word in words {
            let mut curr = &mut head;
            for c in word.into_bytes().into_iter().rev() {
                let index = (c - b'a') as usize;
                curr = curr.next[index].get_or_insert(Box::new(Trie::default()));
            }
            curr.is_word = true;
        }
        Self {
            head,
            queue: vec![],
        }
    }

    fn query(&mut self, letter: char) -> bool {
        let index: usize = (letter as u8 - b'a') as usize;
        self.queue.push(index);
        let mut curr = &self.head;
        for &index in self.queue.iter().rev() {
            if let Some(node) = curr.next[index].as_ref() {
                curr = node.as_ref();
                if curr.is_word {
                    return true;
                }
            } else {
                break;
            }
        }
        false
    }
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut s = StreamChecker::new(vec_string!["cd", "f", "kl"]);
        assert_eq!(s.query('a'), false);
        assert_eq!(s.query('b'), false);
        assert_eq!(s.query('c'), false);
        assert_eq!(s.query('d'), true);
        assert_eq!(s.query('e'), false);
        assert_eq!(s.query('f'), true);
        assert_eq!(s.query('g'), false);
        assert_eq!(s.query('h'), false);
        assert_eq!(s.query('i'), false);
        assert_eq!(s.query('j'), false);
        assert_eq!(s.query('k'), false);
        assert_eq!(s.query('l'), true);
    }
}
