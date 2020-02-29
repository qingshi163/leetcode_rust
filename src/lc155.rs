
#[allow(dead_code)]
struct MinStack {
    v: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            v: Vec::new()
        }
    }

    fn push(&mut self, x: i32) {
        self.v.push(x);
    }

    fn pop(&mut self) {
        self.v.pop();
    }
    
    fn top(&self) -> i32 {
        *self.v.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.v.iter().min().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_155() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);//   --> Returns -2.
    }
}