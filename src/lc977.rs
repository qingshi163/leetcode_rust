
#[allow(dead_code)]
fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; a.len()];
    let mut p = a.binary_search(&0).unwrap_or_else(|x| x);
    while p != 0 && a[p-1] == 0 { p -= 1; }
    let (mut l, mut h) = (p, p);
    for i in 0..a.len() {
        let _h = h;
        result[i] = {
            if l == 0 { h+=1; a[_h]*a[_h] }
            else if h == a.len() { l-=1; a[l]*a[l] }
            else if -a[l-1] < a[h] { l-=1; a[l]*a[l] }
            else { h+=1; a[_h]*a[_h] }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_977() {
        assert_eq!(sorted_squares(vec![-4,-1,0,0,0,3,10]), [0,0,0,1,9,16,100]);
    }
}