use super::substr::*;
use super::matching::*;

#[test]
fn test_common_substring() {
  let s1 = String::from("ABABC");
  let s2 = String::from("BABABD");
  let r = common_substring(s1, s2);
  assert_eq!(r, String::from("ABAB"));
}


#[test]
fn test_matching() {
  let x = String::from("ABCDEFG");
  let y = String::from("CD");
  let v = boyer_moore(x, y);
  assert_eq!(v, Some(vec![2]));
}
