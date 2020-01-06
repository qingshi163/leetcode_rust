#[allow(dead_code)]
fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max = i32::min_value();
    let mut buy = i32::max_value();
    for i in 0..prices.len() {
        buy = std::cmp::min(buy, prices[i]);
        max = std::cmp::max(max, prices[i] - buy);
    }
    if max > 0 {max} else {0}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_121() {
        assert_eq!(max_profit(vec![7,1,5,3,6,4]), 5);
    }
}