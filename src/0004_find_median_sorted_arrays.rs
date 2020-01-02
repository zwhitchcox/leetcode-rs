pub struct Solution;
use std::i32::MAX;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut i, mut j) = (0, 0);
        let total_length = nums1.len() + nums2.len();
        let median = total_length / 2;
        let mut cur = 0;
        let mut last = 0;
        while i + j <= median {
            let num1 = nums1.get(i).or(Some(&MAX)).unwrap();
            let num2 = nums2.get(j).or(Some(&MAX)).unwrap();
            if num1 < num2 {
                i += 1;
                last = cur;
                cur = *num1;
            } else {
                j += 1;
                last = cur;
                cur = *num2;
            }
        }
        
        if total_length % 2 == 0 {
            return (last + cur) as f64 / 2.0;
        }

        cur as f64
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