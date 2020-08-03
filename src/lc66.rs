#[allow(dead_code)]
fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let done = digits.iter_mut().rev().try_for_each(|x| {
        *x += 1;
        *x %= 10;
        if *x != 0 {
            Err(())
        } else {
            Ok(())
        }
    });
    if done.is_ok() {
        digits.insert(0, 1);
    };
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_66() {
        assert_eq!(plus_one(vec![9, 9, 9]), [1, 0, 0, 0]);
        assert_eq!(plus_one(vec![9, 9, 8]), [9, 9, 9]);
    }
}
