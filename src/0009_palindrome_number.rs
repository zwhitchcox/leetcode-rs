pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut y = x;
        let mut z = 0;
        while y > 0 {
            z *= 10;
            z += y % 10;
            y /= 10;
        }
        z == x
    }
}


pub fn main() {
  assert_eq!(Solution::is_palindrome(121), true);
  assert_eq!(Solution::is_palindrome(-121), false);
}
