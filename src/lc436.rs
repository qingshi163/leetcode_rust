#[allow(dead_code)]
fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let mut v: Vec<(i32, usize)> = intervals
        .iter()
        .enumerate()
        .map(|(i, x)| (x[0], i))
        .collect();
    v.sort_unstable();
    intervals
        .iter()
        .map(|interval| {
            let i = v
                .binary_search_by(|x| Ord::cmp(&x.0, &interval[1]))
                .unwrap_or_else(|i| i);
            if i == v.len() {
                -1
            } else {
                v[i].1 as i32
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_right_interval(vec2d![[1, 2]]), [-1]);
        assert_eq!(
            find_right_interval(vec2d![[3, 4], [2, 3], [1, 2]]),
            [-1, 0, 1]
        );
    }
}
