pub struct Solution;
use std::i32::MAX;

fn is_even(n: usize) -> bool {
    n % 2 == 0
}
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        let mid = total_len / 2;
        let mut i = 0;
        let mut j = 0;
        let mut next = || -> i32 {
            let num1 = nums1.get(i).or(Some(&MAX)).unwrap();
            let num2 = nums2.get(j).or(Some(&MAX)).unwrap();
            if num1 < num2 {
                i += 1;
                *num1
            } else {
                j += 1;
                *num2
            }
        };

        let mut cur = 0;
        for _ in 0..mid {
            cur = next();
        }

        if is_even(total_len) {
            (cur + next()) as f64 / 2.0
        } else {
            next() as f64
        }
    }
}

fn main() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
}


fn main() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2]), 2.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
}