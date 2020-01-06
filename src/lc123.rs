#[allow(dead_code)]
fn max_profit(prices: Vec<i32>) -> i32 {
    let (max, lo, hi) = helper(&prices, 0, prices.len(), false);
    if max == 0 {
        return 0;
    }
    let m = vec![
        helper(&prices, 0, lo, false).0,
        helper(&prices, hi - 1, lo, true).0,
        helper(&prices, hi + 1, prices.len(), false).0,
    ];
    max + m.iter().max().unwrap()
}
fn helper(prices: &[i32], start: usize, end: usize, rev: bool) -> (i32, usize, usize) {
    let mut max = 0;
    let mut buy = i32::max_value();
    let (mut lo, mut hi) = (0, 0);
    let mut tmp_lo = 0;
    let mut i = start;
    while i != end {
        if prices[i] < buy {
            buy = prices[i];
            tmp_lo = i;
        }
        if prices[i] - buy > max {
            max = prices[i] - buy;
            lo = tmp_lo;
            hi = i;
        }
        if rev {
            i -= 1;
        } else {
            i += 1;
        }
    }
    (max, lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 13);
    }
}
