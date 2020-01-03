impl Solution {pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
      let (mut last, mut sum) = (0, 0);
      for c in s.chars() {
        let cur = match c {
          'I' => 1,
          'V' => 5,
          'X' => 10,
          'L' => 50,
          'C' => 100,
          'D' => 500,
          'M' => 1000,
          _ => 0,
        };
        if cur > last {
          sum -= last * 2;
        }
        sum += cur;
        last = cur;
      }
      sum
    }
}

pub fn main() {
  assert_eq!(
    3,
    Solution::roman_to_int(String::from("III")),
  );
  assert_eq!(
    4,
    Solution::roman_to_int(String::from("IV")),
  );
  assert_eq!(
    9,
    Solution::roman_to_int(String::from("IX")),
  );
  assert_eq!(
    58,
    Solution::roman_to_int(String::from("LVIII")),
  );
}