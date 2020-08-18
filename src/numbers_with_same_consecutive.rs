fn dfs(num: &mut [i32], i: usize, k: i32, ret: &mut Vec<i32>) {
    if i == num.len() {
        ret.push(num.iter().fold(0, |a, x| a * 10 + x));
        return;
    }
    let prev = num[i - 1];
    for d in (0..=9).filter(|&x| (x - prev).abs() == k) {
        num[i] = d;
        dfs(num, i + 1, k, ret);
    }
}

#[allow(dead_code)]
fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
    let n = n as usize;
    let mut ret: Vec<i32> = vec![];
    if n == 0 { return ret; }
    if n == 1 { ret.push(0); }
    for first in 1..=9 {
        let mut num: Vec<i32> = vec![0; n];
        num[0] = first;
        dfs(&mut num, 1, k, &mut ret);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(nums_same_consec_diff(3, 7), vec![181, 292, 707, 818, 929]);
        assert_eq!(nums_same_consec_diff(1, 0), vec![0,1,2,3,4,5,6,7,8,9]);
        assert_eq!(
            nums_same_consec_diff(2, 1),
            vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98]
        );
    }
}
