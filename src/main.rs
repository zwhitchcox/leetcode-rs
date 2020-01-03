
pub struct Solution;

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows < 2 {
      return s;
    }
    let n = s.len() as i32;
    let cycle_len = num_rows * 2 - 2;
    let mut result = String::new();
    let sv: Vec<_> = s.chars().collect();

    for row in 0..num_rows {
      let mut cycle = 0;
      while cycle + row < n {
        result.push(sv[(cycle + row) as usize]);
        if row != 0 && row != num_rows - 1 && cycle + cycle_len - row < n {
          result.push(sv[(cycle + cycle_len - row) as usize]);
        }
        cycle += cycle_len;
      }
    }
    result
  }
}

pub fn main() {
  assert_eq!(
    String::from("PAHNAPLSIIGYIR"),
    Solution::convert("PAYPALISHIRING".to_string(), 3),
  );
  assert_eq!(
    String::from("PYAIHRNAPLSIIG"),
    Solution::convert("PAYPALISHIRING".to_string(), 2),
  );
  assert_eq!(
    String::from("ABC"),
    Solution::convert("ABC".to_string(), 3),
  );
  assert_eq!(
    String::from("PINALSIGYAHRPI"),
    Solution::convert("PAYPALISHIRING".to_string(), 4),
  );
  assert_eq!(
    String::from("AB"),
    Solution::convert("AB".to_string(), 1),
  );
  assert_eq!(
    String::from("Aaidoeswr,haenme,rtesqecouishtabrateaeaietedrcinwtgnrlloacsoajsmnsoucutoadodiiesplnrmiaodprs,ubroohreunefnttacneedhsmwynihrieto,iheeaalwnefrdutettpntainnwrdvdr."),
    Solution::convert("Apalindromeisaword,phrase,number,orothersequenceofunitsthatcanbereadthesamewayineitherdirection,withgeneralallowancesforadjustmentstopunctuationandworddividers.".to_string(), 2),
  );
}