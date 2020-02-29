
use std::collections::HashSet;
#[allow(dead_code)]
fn is_happy(n: i32) -> bool {
    let mut n = n;
    let mut set = HashSet::new();
    loop {
        if n == 1 { return true; }
        if !set.insert(n) {
            return false;
        }
        let mut nn = 0;
        while n != 0 {
            nn += (n % 10) * (n % 10);
            n /= 10;
        }
        n = nn;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_202() {
        assert!(is_happy(19));
        assert!(!is_happy(18));
    }
}