pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut longest, mut start) = (0, 0);
        let mut hash: HashMap<char, usize> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            if let Some(&j) = hash.get(&c)  {
                if j >= start {
                    longest = longest.max(i - start);
                    start = j + 1;
                }
            }
            hash.insert(c, i);
        }
        longest.max(s.len() - start) as i32
    }
 }

fn main() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("bba")),
        2
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abcabcbb")),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abba")),
        2
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("cdd")),
        2
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from(" ")),
        1
    );
}
