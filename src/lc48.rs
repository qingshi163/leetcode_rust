
#[allow(dead_code)]
fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    matrix.reverse();
    for i in 0..n {
        for j in i+1..n {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_48() {
        let mut m = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        rotate(&mut m);
        assert_eq!(m, [[7,4,1],[8,5,2],[9,6,3]]);
    }
}