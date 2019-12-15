#[allow(dead_code)]
fn convert(s: String, n: i32) -> String {
    let it = (0..n)
        .into_iter() // 0,1,2,3
        .chain((1..=n - 2).rev().into_iter()); // 0,1,2,3,2,1
    let mut vr: Vec<Vec<char>> = vec![vec![]; n as usize];
    s.chars()
        .zip(it.cycle())
        .for_each(|(c, i)| vr[i as usize].push(c));
    vr.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lc6() {
        assert_eq!(convert("PAYPALISHIRING".to_owned(), 4), "PINALSIGYAHRPI");
        assert_eq!(convert("PAYPALISHIRING".to_owned(), 3), "PAHNAPLSIIGYIR");
    }
}
