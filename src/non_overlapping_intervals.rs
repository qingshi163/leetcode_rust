/*
Given a collection of intervals, find the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.



Example 1:

Input: [[1,2],[2,3],[3,4],[1,3]]
Output: 1
Explanation: [1,3] can be removed and the rest of intervals are non-overlapping.

Example 2:

Input: [[1,2],[1,2],[1,2]]
Output: 2
Explanation: You need to remove two [1,2] to make the rest of intervals non-overlapping.

Example 3:

Input: [[1,2],[2,3]]
Output: 0
Explanation: You don't need to remove any of the intervals since they're already non-overlapping.



Note:

    You may assume the interval's end point is always bigger than its start point.
    Intervals like [1,2] and [2,3] have borders "touching" but they don't overlap each other.


 */

#[allow(dead_code)]
fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable();
    let mut ret = 0;
    let mut end = i32::max_value();
    for interval in intervals.iter().rev() {
        if interval[1] > end {
            ret += 1;
        } else {
            end = interval[0];
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_overlapping_intervals() {
        assert_eq!(
            erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
        assert_eq!(
            erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2]]),
            3
        );
        assert_eq!(erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]), 0);
        assert_eq!(
            erase_overlap_intervals(vec![
                vec![0, 2],
                vec![1, 3],
                vec![1, 3],
                vec![2, 4],
                vec![3, 5],
                vec![3, 5],
                vec![4, 6]
            ]),
            4
        );
    }
}
