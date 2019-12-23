
use std::cmp::Ordering::*;
#[allow(dead_code)]
fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let len = nums.len();
    if len < 4 { return results; }
    nums.sort_unstable();
    for a in 0..len - 3 {
        if a != 0 && nums[a] == nums[a-1] { continue; }
        for b in a+1..len - 2 {
            if b != a+1 && nums[b] == nums[b-1] { continue; }
            let mut c = b + 1;
            let mut d = len - 1;
            while c < d {
                match (nums[a]+nums[b]+nums[c]+nums[d]).cmp(&target) {
                    Less => { c+=1; }
                    Greater => { d-=1; }
                    Equal => {
                        results.push(vec![nums[a], nums[b], nums[c], nums[d]]);
                        while c < d && nums[c] == nums[c+1] { c+=1; }
                        while c < d && nums[d] == nums[d-1] { d-=1; }
                        c+=1; d-=1;
                    }
                }
            }
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_18() {
        assert_eq!(four_sum(vec![1,0,-1,0,-2,2], 0), [
            [-2,-1,1,2],
            [-2,0,0,2],
            [-1,0,0,1],
        ]);
    }
}
