pub struct Solution;

impl Solution {
  pub fn my_atoi(str: String) -> i32 {
    let mut chrs = str
      .chars()
      .skip_while(|c| c == &' ')
      .peekable();
    let mut sign = 1;
    let mut def = std::i32::MAX;
    match chrs.peek() {
      Some(&'-') => {
        sign = -1;
        def = std::i32::MIN;
        chrs.next();
      }
      Some(&'+') => {
        chrs.next();
      }
      None => {
        return 0;
      }
      _ => {}
    }
    match chrs.peek() {
      Some(&c) => {
        if !c.is_ascii_digit() {
          return 0;
        }
      },
      None => {
        return 0;
      }
    }
    chrs.take_while(|c| c.is_ascii_digit())
      .collect::<String>()
      .parse::<i32>()
      .map(|n| n * sign)
      .unwrap_or(def)
  }
}

pub fn main() {
  assert_eq!(Solution::my_atoi("-42".to_string()), -42);
  assert_eq!(Solution::my_atoi("".to_string()), 0);
  assert_eq!(Solution::my_atoi("   4193 with words".to_string()), 4193);
  assert_eq!(Solution::my_atoi("words and 4193".to_string()), 0);
  assert_eq!(Solution::my_atoi("-91283472332".to_string()), std::i32::MIN);
  assert_eq!(
    Solution::my_atoi("9223372036854775808".to_string()),
    std::i32::MAX
  );
}

mod test {
  #[test]
  fn basics() {}
}