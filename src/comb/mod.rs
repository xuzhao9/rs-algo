//
// Permuation and Combination
// Author: xuzhao9 (i@@xuzhao.net)
//

use std::collections::HashSet;

pub fn permutation(m: i32, n: i32) -> Option<Vec<HashSet<i32>>> {
  if n == 0 { return None; }
  Some(Vec::new())
}

// Select a group of n elements from 1..m
pub fn combination(m: Vec<i32>, n: i32) -> Option<Vec<HashSet<i32>>> {
  if n <= 0 {
    return None;
  }
  let mut results: Vec<HashSet<i32>> = Vec::new();
  for e in m {
    let mut h: HashSet<i32> = HashSet::new();
    h.insert(e);
    results.push(h);
  }
  for num in 2..n {
    // now calculate (m, 1)
    let mut rest = num;
    let mut cur:HashSet<i32> = HashSet::new();
  }
  Some(results)
}
