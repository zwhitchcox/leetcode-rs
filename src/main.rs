pub struct Solution;
use std::i32::MAX;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = nums1;
        nums.extend(nums2);
        nums.sort();
        let median = nums.len() / 2;

        
        if nums.len() % 2 == 0 {
            return (nums[median - 1] + nums[median]) as f64 / 2.0;
        }

        nums[median] as f64
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