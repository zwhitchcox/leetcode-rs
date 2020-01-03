pub struct Solution;

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
  }
}
/*
PYAIHRN
AAPLSIIG
*/
pub fn main() {
  assert_eq!(
    String::from("Aaidoeswr,haenme,rtesqecouishtabrateaeaietedrcinwtgnrlloacsoajsmnsoucutoadodiiesplnrmiaodprs,ubroohreunefnttacneedhsmwynihrieto,iheeaalwnefrdutettpntainnwrdvdr."),
    Solution::convert("Apalindromeisaword,phrase,number,orothersequenceofunitsthatcanbereadthesamewayineitherdirection,withgeneralallowancesforadjustmentstopunctuationandworddividers.".to_string(), 2),
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
    String::from("PAHNAPLSIIGYIR"),
    Solution::convert("PAYPALISHIRING".to_string(), 3),
  );
  assert_eq!(
    String::from("PINALSIGYAHRPI"),
    Solution::convert("PAYPALISHIRING".to_string(), 4),
  );
  assert_eq!(
    String::from("AB"),
    Solution::convert("AB".to_string(), 1),
  );
}

/* alternative */
// impl Solution {
//   pub fn convert(s: String, num_rows: i32) -> String {
//     let num_rows = num_rows as usize;
//     if num_rows < 2 {
//       return s;
//     }
//     let mut rows = vec![String::new(); num_rows as usize];
//     let mut cur_row = 0;
//     let mut down = true;
//     for c in s.chars() {
//       rows[cur_row].push(c);
//       cur_row = if down { cur_row + 1 } else { cur_row - 1 };
//       down = down == (cur_row > 0 && cur_row < num_rows - 1);
//     }
//     rows.concat()
//   }
// }
