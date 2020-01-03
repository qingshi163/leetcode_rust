
#[allow(dead_code)]
fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return Vec::new();
    }
    let mut output = Vec::new();
    let (mut start_x, mut start_y, mut end_x, mut end_y) = (0, 0, matrix[0].len(), matrix.len());
    while start_x < end_x && start_y < end_y {
        helper(&matrix, start_x, start_y, end_x, end_y, &mut output);
        println!("{},{},{},{} :{:?}", start_x, start_y, end_x, end_y, output);
        start_x += 1; start_y += 1; end_x -= 1; end_y -= 1;
    }
    output
}

fn helper(matrix: &[Vec<i32>], start_x: usize, start_y: usize, end_x: usize, end_y: usize, output: &mut Vec<i32>) {
    for x in start_x..end_x {
        output.push(matrix[start_y][x]);
    }
    for y in start_y+1..end_y {
        output.push(matrix[y][end_x-1]);
    }
    if start_y != end_y - 1 {
        for x in (start_x..end_x-1).rev() {
            output.push(matrix[end_y-1][x]);
        }
    }
    if start_x != end_x - 1 {
        for y in (start_y+1..end_y-1).rev() {
            output.push(matrix[y][start_x]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_54() {
        assert_eq!(spiral_order(vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ]), [1,2,3,6,9,8,7,4,5]);
        assert_eq!(spiral_order(vec![
            vec![1,2,3,4],
            vec![5,6,7,8]
        ]), [1,2,3,4,8,7,6,5]);
        assert_eq!(spiral_order(vec![
            vec![1,2,3,4],
            vec![5,6,7,8],
            vec![9,10,11,12]
        ]), [1,2,3,4,8,12,11,10,9,5,6,7]);
    }
}