pub struct Solution;
use std::convert::TryFrom;

impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let mut r = 0;
    let mut x = x as i64;
    while x.abs() > 0 {
      r *= 10;
      r += x % 10;
      x /= 10
    }

    i32::try_from(r).unwrap_or(0)
  }
}

pub fn main() {
  assert_eq!(Solution::reverse(123), 321);
  assert_eq!(Solution::reverse(-321), -123);
  assert_eq!(Solution::reverse(1534236469), 0);
}

