#[allow(dead_code)]
fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    let mut cur = prices[0];
    let mut profit = 0;
    for i in 1..prices.len() {
        if prices[i] > cur {
            profit += prices[i] - cur;
        }
        cur = prices[i];
    }
    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_122() {
        assert_eq!(max_profit(vec![7,1,5,3,6,4]), 7);
    }
}