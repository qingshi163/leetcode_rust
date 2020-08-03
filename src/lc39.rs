
#[allow(dead_code)]
fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    // let mut stack = Vec::new();
    if candidates.is_empty() || target < 0 {
        return results;
    }
    candidates.sort_unstable();
    helper(&mut results, Vec::new(), &candidates, 0, target);
    results
}

fn helper(
    results: &mut Vec<Vec<i32>>,
    stack: Vec<i32>,
    candidates: &[i32],
    start_idx: usize,
    target: i32
) {
    for i in start_idx..candidates.len() {
        let val = candidates[i];
        if target < val {
            break;
        }
        let mut new_stack = stack.clone();
        new_stack.push(val);
        if target == val {
            results.push(new_stack);
        } else {
            helper(results, new_stack, candidates, i, target-val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_39() {
        assert_eq!(
            combination_sum(vec![1], 8),
            vec![vec![1,1,1,1,1,1,1,1]]
        );
        assert_eq!(
            combination_sum(vec![2,3,6,7], 7),
            vec![
                vec![2,2,3],
                vec![7],
            ]
        );
        assert_eq!(
            combination_sum(vec![2,3,5], 8),
            vec![
                vec![2,2,2,2],
                vec![2,3,3],
                vec![3,5],
            ]
        );
    }
}