/*
983. Minimum Cost For Tickets
Medium

1710

33

Add to List

Share
In a country popular for train travel, you have planned some train travelling one year in advance.  The days of the year that you will travel is given as an array days.  Each day is an integer from 1 to 365.

Train tickets are sold in 3 different ways:

a 1-day pass is sold for costs[0] dollars;
a 7-day pass is sold for costs[1] dollars;
a 30-day pass is sold for costs[2] dollars.
The passes allow that many days of consecutive travel.  For example, if we get a 7-day pass on day 2, then we can travel for 7 days: day 2, 3, 4, 5, 6, 7, and 8.

Return the minimum number of dollars you need to travel every day in the given list of days.



Example 1:

Input: days = [1,4,6,7,8,20], costs = [2,7,15]
Output: 11
Explanation:
For example, here is one way to buy passes that lets you travel your travel plan:
On day 1, you bought a 1-day pass for costs[0] = $2, which covered day 1.
On day 3, you bought a 7-day pass for costs[1] = $7, which covered days 3, 4, ..., 9.
On day 20, you bought a 1-day pass for costs[0] = $2, which covered day 20.
In total you spent $11 and covered all the days of your travel.
Example 2:

Input: days = [1,2,3,4,5,6,7,8,9,10,30,31], costs = [2,7,15]
Output: 17
Explanation:
For example, here is one way to buy passes that lets you travel your travel plan:
On day 1, you bought a 30-day pass for costs[2] = $15 which covered days 1, 2, ..., 30.
On day 31, you bought a 1-day pass for costs[0] = $2 which covered day 31.
In total you spent $17 and covered all the days of your travel.


Note:

1 <= days.length <= 365
1 <= days[i] <= 365
days is in strictly increasing order.
costs.length == 3
1 <= costs[i] <= 1000
*/

#[allow(dead_code)]
fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = vec![0; days.len()];
        dfs(days.as_slice(), costs.as_slice(), 0, &mut dp)
}

fn dfs(days: &[i32], costs: &[i32], i: usize, dp: &mut Vec<i32>) -> i32 {
    let today = match days.get(i) {
        None => return 0,
        Some(day) => day
    };
    if dp[i] == 0 {
        let mut forward = i + 1;
        let one = costs[0] + dfs(days, costs, forward, dp);
        while forward < days.len() {
            if days[forward] >= today + 7 {
                break;
            }
            forward += 1;
        }
        let seven = costs[1] + dfs(days, costs, forward, dp);
        while forward < days.len() {
            if days[forward] >= today + 30 {
                break;
            }
            forward += 1;
        }
        let thirty = costs[2] + dfs(days, costs, forward, dp);

        use std::cmp::min;
        dp[i] = min(min(one, seven), thirty);
    }
    dp[i]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]), 11);
        assert_eq!(
            mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
            17
        );
        assert_eq!(
            mincost_tickets(
                vec![
                    3, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 20, 21, 23, 25, 26, 27, 29, 30, 33,
                    34, 35, 36, 38, 39, 40, 42, 45, 46, 47, 48, 49, 51, 53, 54, 56, 57, 58, 59, 60,
                    61, 63, 64, 67, 68, 69, 70, 72, 74, 77, 78, 79, 80, 81, 82, 83, 84, 85, 88, 91,
                    92, 93, 96
                ],
                vec![3, 17, 57]
            ),
            170
        );
    }
}
