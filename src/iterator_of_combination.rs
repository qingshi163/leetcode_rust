struct CombinationIterator {
    characters: Vec<u8>,
    current: Vec<usize>,
    has_next: bool,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl CombinationIterator {

    #[allow(non_snake_case)]
    fn new(characters: String, combinationLength: i32) -> Self {
        Self {
            characters: characters.into_bytes(),
            current: (0..combinationLength as usize).collect(),
            has_next: true
        }
    }

    fn next(&mut self) -> String {
        let ret = unsafe { String::from_utf8_unchecked(self.current.iter().map(|&x| self.characters[x]).collect()) };
        for i in (0..self.current.len()).rev() {
            if self.current[i] < self.characters.len() - self.current.len() + i {
                self.current[i] += 1;
                for j in (i + 1)..self.current.len() {
                    self.current[j] = self.current[j - 1] + 1;
                }
                return ret;
            }
        }
        self.has_next = false;
        ret
    }

    fn has_next(&self) -> bool {
        // self.current[0] < self.characters.len() - self.current.len()
        self.has_next
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_of_combination() {
        let mut obj = CombinationIterator::new("abc".into(), 2);
        assert_eq!(obj.next(), "ab");
        assert!(obj.has_next());
        assert_eq!(obj.next(), "ac");
        assert!(obj.has_next());
        assert_eq!(obj.next(), "bc");
        assert!(!obj.has_next());
    }
}