use std::rc::Rc;
use std::cell::RefCell;
use super::utils::*;

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
    
    fn add(&self, key: i32) {
        
    }
    
    fn remove(&self, key: i32) {
        
    }
    
    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {

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