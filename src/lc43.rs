
#[allow(dead_code)]
fn multiply(num1: String, num2: String) -> String {
    let mut res = vec![0u8; num1.len() + num2.len()];
    let num1: Vec<u8> = num1.bytes().map(|b| b - b'0').collect();
    let num2: Vec<u8> = num2.bytes().map(|b| b - b'0').collect();
    for i in (0..num1.len()).rev() {
        let mut carry = 0;
        for j in (0..num2.len()).rev() {
            let tmp = num1[i] * num2[j] + res[i+j+1] + carry;
            carry = tmp / 10;
            res[i+j+1] = tmp % 10;
        }
        res[i] += carry;
    }
    match res.iter().position(|&b| b != 0) {
        Some(i) => {
            let res = &mut res[i..];
            res.iter_mut().for_each(|b| *b += b'0');
            unsafe {
                String::from_utf8_unchecked(res.to_vec())
            }
        },
        None => {
            "0".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {
        assert_eq!(multiply("123".to_owned(), "123".to_owned()), "15129");
        assert_eq!(multiply("1".to_owned(), "0".to_owned()), "0");
        assert_eq!(multiply("999".to_owned(), "999".to_owned()), "998001");
    }
}