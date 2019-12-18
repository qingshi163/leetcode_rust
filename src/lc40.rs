
#[allow(dead_code)]
fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let mut stack = Vec::new();
    if candidates.is_empty() || target < 0 {
        return results;
    }
    let mut candidates = candidates;
    candidates.sort_unstable();
    helper(&mut results, &mut stack, &candidates, 0, target);
    results
}

fn helper(
    results: &mut Vec<Vec<i32>>,
    stack: &mut Vec<i32>,
    candidates: &Vec<i32>,
    start_idx: usize,
    target: i32
) {
    if target == 0 {
        results.push(stack.clone());
        return;
    }
    let len = candidates.len();
    let mut i = start_idx;
    while i < len {
        let val = candidates[i];
        if target >= val {
            stack.push(val);
            helper(results, stack, candidates, i + 1, target - val);
            stack.pop();
        }
        while i < len && candidates[i] == val {
            i+=1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_40() {
        let results = vec![
            vec![1,1,6],
            vec![1,2,5],
            vec![1,7],
            vec![2,6],
        ];
        assert_eq!(results, combination_sum2(vec![10,1,2,7,6,1,5], 8));
        let results = vec![
            vec![1,2,2],
            vec![5],
        ];
        assert_eq!(results, combination_sum2(vec![2,5,2,1,2], 5));
    }
}