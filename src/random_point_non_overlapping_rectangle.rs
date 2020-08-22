use rand::prelude::*;
struct Solution {
    rects: Vec<Vec<i32>>,
    linear_volumes: Vec<u64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut acc = 0;
        let linear_volumes = rects
            .iter()
            .map(|rect| {
                let w = (rect[0] - rect[2]).abs() as u64 + 1;
                let h = (rect[1] - rect[3]).abs() as u64 + 1;
                acc += w * h;
                acc
            })
            .collect();
        Self {
            rects,
            linear_volumes,
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        let last = match self.linear_volumes.last() {
            None => return vec![],
            Some(&last) => last,
        };
        let r = rand::thread_rng().gen_range(1, last + 1);
        let index = self.linear_volumes.binary_search(&r).unwrap_or_else(|x| x);
        let start = if index != 0 {
            self.linear_volumes[index - 1]
        } else {
            0
        };
        let r = (r - start - 1) as i32;
        let rect = &self.rects[index];
        let w = (rect[0] - rect[2]).abs() + 1;
        vec![rect[0] + r % w, rect[1] + r / w]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut s = Solution::new(vec2d![
            [82918473, -57180867, 82918476, -57180863],
            [83793579, 18088559, 83793580, 18088560],
            [66574245, 26243152, 66574246, 26243153],
            [72983930, 11921716, 72983934, 11921720]
        ]);
        for _ in 0..100 {
            println!("{:?}", s.pick());
        }
    }
}
