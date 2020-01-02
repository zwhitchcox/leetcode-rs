pub struct Solution;

impl Solution {
  pub fn longest_palindrome(sv: String) -> String {
    let s: Vec<char> = sv.chars().collect();
    let len = s.len();
    if len == 0 {
        return "".to_string();
    }
    let mut start = 0;
    let mut end = 0;
    for i in 0..s.len() {
        let mut left = i;
        let mut right = i;
        while right + 1 < len && s[right + 1] == s[left] {
            right += 1;
        }
        while right + 1 < len && left > 0 && s[right + 1] == s[left -1] {
            right += 1;
            left -= 1;
        }
        
        if right - left > end - start {
            end = right;
            start = left;
        }
    }
    s[start..=end].iter().collect()
  }
}

fn main() {
    assert_eq!(
        Solution::longest_palindrome(String::from("badad")),
        String::from("ada")
    );
    assert_eq!(
        Solution::longest_palindrome(String::from("ssaasdbc")),
        String::from("saas")
    );
    assert_eq!(
        Solution::longest_palindrome(String::from("tattarrattat")),
        String::from("tattarrattat")
    );
}
