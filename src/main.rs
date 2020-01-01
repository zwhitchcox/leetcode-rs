pub struct Solution;
use std::i32::MAX;

fn is_even(n: usize) -> bool {
    n % 2 == 0
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut result = vec![];
        let mut i = 0;
        let mut j = 0;
        for _ in 0..(nums1.len() + nums2.len()) {
            let num1 = nums1[i];
            let num2 = nums2[j];
            if num1 > num2 {
                j += 1;
                result.push(num2);
            } else {
                i += 1;
                result.push(num1);
            }
        }
        let median = result.len() / 2;
        if is_even(result.len()) {
            return (result[median] + result[median]) as f64 / 2.0;
        }
        return result[median + 1] as f64
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