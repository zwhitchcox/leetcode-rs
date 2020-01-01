pub struct Solution;

impl Solution {
  pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut prod = 1;
    let mut sum = 0;
    let mut n = n;
    while n != 0 {
      let cur = n % 10;
      n /= 10;
      prod *= cur;
      sum += cur;
    }
    
    prod - sum
  }
}



fn main() {
    assert_eq!(Solution::subtract_product_and_sum(234), 15);
    assert_eq!(Solution::subtract_product_and_sum(234), 15);
}