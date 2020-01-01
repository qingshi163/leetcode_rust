
#[allow(dead_code)]
fn add_binary(a: String, b: String) -> String {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut apos = a.len();
    let mut bpos = b.len();
    let mut bits = vec![];
    let mut carry = false;
    while apos != 0 || bpos != 0 || carry {
        let va = if apos != 0 { apos-=1; a[apos]==b'1' } else {false};
        let vb = if bpos != 0 { bpos-=1; b[bpos]==b'1' } else {false};
        bits.push(va ^ vb ^ carry);
        carry = if carry { va | vb } else { va & vb };
    }
    unsafe {
        String::from_utf8_unchecked(bits.iter().rev().map(|&x| if x {b'1'} else {b'0'}).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_67() {
        assert_eq!(add_binary("1011".to_owned(), "1".to_owned()), "1100");
        assert_eq!(add_binary("1011".to_owned(), "1100".to_owned()), "10111");
    }
}