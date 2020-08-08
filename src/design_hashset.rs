
#[derive(Clone)]
enum Node {
    Nil,
    One(i32),
    More(Vec<i32>)
}

struct MyHashSet {
    length: usize,
    bulk: Vec<Node>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyHashSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            length: 0,
            bulk: vec![]
        }
    }
    fn with_capacity(capacity: usize) -> Self {
        Self {
            length: 0,
            bulk: vec![Node::Nil; capacity]
        }
    }

    fn rebuild(&mut self, capacity: usize) {
        let mut new_self = MyHashSet::with_capacity(capacity);
        for node in &self.bulk {
            match node {
                Node::Nil => {},
                Node::One(val) => new_self.add(*val),
                Node::More(v) => v.iter().for_each(|&x| new_self.add(x))
            }
        }
        self.bulk = new_self.bulk
    }

    fn add(&mut self, key: i32) {
        if self.contains(key) { return; }
        if self.length * 2 >= self.bulk.len() {
            self.rebuild(std::cmp::max(self.bulk.len() * 2, 8));
        }
        let index = self.hash(key);
        match &mut self.bulk[index] {
            Node::Nil => self.bulk[index] = Node::One(key),
            Node::One(val) => self.bulk[index] = Node::More(vec![*val, key]),
            Node::More(v) => v.push(key)
        }
        self.length += 1;
    }

    fn remove(&mut self, key: i32) {
        if self.length == 0 { return; }
        let index = self.hash(key);
        match &mut self.bulk[index] {
            Node::Nil => {},
            Node::One(val) => {
                if *val == key {
                    self.bulk[index] = Node::Nil;
                    self.length -= 1;
                }
            },
            Node::More(v) => {
                for i in 0..v.len() {
                    if v[i] == key {
                        v.swap_remove(i);
                        self.length -= 1;
                        break;
                    }
                }
            }
        }
    }
    
    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        if self.length == 0 { return false; }
        let index = self.hash(key);
        match &self.bulk[index] {
            Node::Nil => false,
            Node::One(val) => *val == key,
            Node::More(v) => v.iter().any(|&x| x == key)
        }
    }

    fn hash(&self, key: i32) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{ Hash, Hasher };
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.bulk.len() as u64) as usize
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_design_hashset() {
    }
}