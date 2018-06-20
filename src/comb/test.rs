use super::*;

const THRESHOLD: i32 = 11;

fn factorial(v: i32) -> i32 {
  let mut r = 1;
  for x in 1..v+1 {
    r = r * x;
  }
  r
}

fn cal_permutation_len(m: i32, n: i32) -> i32 {
  factorial(m) / factorial(m - n)
}

fn cal_combination_len(m: i32, n: i32) -> i32 {
  factorial(m) / (factorial(m - n) * factorial(n))
}

#[test]
fn test_combination() {
  for m in 1..THRESHOLD {
    for n in 1..m {
      assert_eq!(combination(m,n).unwrap().len() as i32, cal_combination_len(m, n));
    }
  }
}

#[test]
fn test_permutation() {
  for m in 1..THRESHOLD {
    for n in 1..m {
      assert_eq!(permutation(m,n).unwrap().len() as i32, cal_permutation_len(m, n));
    }
  }
}

