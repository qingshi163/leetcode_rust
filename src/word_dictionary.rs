#[derive(Clone)]
struct Node {
    next: Vec<Option<Node>>,
    is_word: bool
}
impl Node {
    pub fn new() -> Node {
        Node {
            next: vec![None; 26],
            is_word: false
        }
    }
}

struct WordDictionary {
    head: Node
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl WordDictionary {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            head: Node::new()
        }
    }
    
    /** Adds a word into the data structure. */
    fn add_word(&mut self, word: String) {
        let word = word.as_bytes();
        let mut node = &mut self.head;
        for c in word {
            node = node.next[(c - b'a') as usize].get_or_insert(Node::new());
        }
        node.is_word = true;
    }

    fn _search(&self, head: &Node, word: &[u8]) -> bool {
        let mut node = head;
        for i in 0..word.len() {
            let c = word[i];
            if c == b'.' {
                return node.next.iter().any(|x| {
                    x.as_ref().map_or(false, |next| self._search(next, &word[i + 1..]))
                });
            }
            match &node.next[(c - b'a') as usize] {
                None => return false,
                Some(next) => node = next
            }
        }
        node.is_word
    }
    
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    fn search(&self, word: String) -> bool {
        let word = word.as_bytes();
        self._search(&self.head, word)
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_dictionary() {
        let mut dict = WordDictionary::new();
        dict.add_word("bad".to_owned());
        dict.add_word("dad".to_owned());
        dict.add_word("mad".to_owned());
        assert!(!dict.search("pad".to_owned()));
        assert!(dict.search("bad".to_owned()));
        assert!(dict.search(".ad".to_owned()));
        assert!(dict.search("b..".to_owned()));
        assert!(!dict.search("b...".to_owned()));
    }
}