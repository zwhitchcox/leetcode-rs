pub struct Solution;

impl Solution {
  pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for num in nums {
      let num_digits = (num as f64).log10().floor() as i32 + 1;
      if num_digits % 2 == 0 {
        count += 1;
      }
    }
    count
  }
}




fn main() {
    assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
}