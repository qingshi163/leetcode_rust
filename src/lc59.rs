#[allow(dead_code)]
fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut shift = 0;
    let mut num = 1;
    let mut n = n as usize;
    let mut ret = vec![vec![0; n]; n];
    loop {
        for i in 0..n {
            ret[shift][shift + i] = num;
            num += 1;
        }
        for i in 1..n {
            ret[shift + i][shift + n - 1] = num;
            num += 1;
        }
        for i in (0..n - 1).rev() {
            ret[shift + n - 1][shift + i] = num;
            num += 1;
        }
        for i in (1..n - 1).rev() {
            ret[shift + i][shift] = num;
            num += 1;
        }
        shift += 1;
        if n <= 2 {
            break;
        }
        n -= 2;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(generate_matrix(3), vec2d![[1, 2, 3], [8, 9, 4], [7, 6, 5]]);
        assert_eq!(generate_matrix(2), vec2d![[1, 2], [4, 3]]);
    }
}
