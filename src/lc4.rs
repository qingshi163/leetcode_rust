

fn find_median_single(nums: &Vec<i32>, head: usize, tail: usize) -> f64 {
    let dif = tail - head;
    let mid = dif / 2;
    
    if dif & 1 != 0 {
        (nums[head + mid - 1] + nums[head + mid]) as f64 / 2.0
    } else {
        nums[head + mid - 1] as f64
    }
}

#[allow(dead_code)]
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.is_empty() {
        return find_median_single(&nums2, 1, nums2.len());
    }
    if nums2.is_empty() {
        return find_median_single(&nums1, 1, nums1.len());
    }
    let (mut head1, mut head2, mut tail1, mut tail2) = (1, 1, nums1.len(), nums2.len());
    while (tail1 - head1) + (tail2 - head2) > 0 {
        if nums1[head1-1] < nums2[head2-1] {
            head1+=1;
        } else {
            head2+=1;
        }
        if nums1[tail1-1] > nums2[tail2-1] {
            tail1-=1;
        } else {
            tail2-=1;
        }
        if head1 > tail1 {
            return find_median_single(&nums2, head2, tail2);
        }
        if head2 > tail2 {
            return find_median_single(&nums1, head1, tail1);
        }
    }
    (nums1[head1-1] + nums2[head2-1]) as f64 / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(find_median_sorted_arrays(vec![0,4], vec![2,3]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![1,2,3,4,5], vec![]), 3.0);
        assert_eq!(find_median_sorted_arrays(vec![1,2,3], vec![4,5]), 3.0);
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3,4,5]), 3.0);
        assert_eq!(find_median_sorted_arrays(vec![1,2,3], vec![3,4,5]), 3.0);
        assert_eq!(find_median_sorted_arrays(vec![1], vec![1]), 1.0);
        assert_eq!(find_median_sorted_arrays(vec![1], vec![2]), 1.5);
        assert_eq!(find_median_sorted_arrays(vec![1,5], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![1,2,2], vec![1,2,3]), 2.0);
    }
}