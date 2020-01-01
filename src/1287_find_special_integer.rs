pub struct Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let len = arr.len() as f64;
        let mut cur = arr[0];
        let mut count = 1;
        for &num in arr.iter().skip(1) {
            if num != cur {
                cur = num;
                count = 1;
            } else {
                count += 1
            }
            if count as f64 / len > 0.25 {
                return num;
            }
        }
        cur
    }
}

fn main() {
    assert_eq!(
        Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
        6
    );
    assert_eq!(Solution::find_special_integer(vec![1, 2, 3, 3]), 3);
}
