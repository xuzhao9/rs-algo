//
// String matching algorithms
// Author: xuzhao9 (i@xuzhao.net)
//

use std::collections::HashMap;

// If no match, return none
// Otherwise, return Some(index), where index is the first match
pub fn boyer_moore(y: String, x: String) -> Option<Vec<usize>> {
  let gs = bm_good_shift(&x);
  let bs = bm_bad_shift(&x);
  let mut j: i32 = 0;
  let mut i: i32 = 0;
  if y.len() < x.len() {
    return None;
  }
  let mut result: Vec<usize> = Vec::new();
  while j <= (y.len() as i32 - x.len() as i32) {
    for i in (0..x.len() as i32).rev() {
      if x[i as usize] != y[(i + j) as usize] {
        break;
      }
    }
    if i < 0 {
      result.push(j);
      j += gs[0];
    } else {
      j += max(gs[i], bs[y[i+j]] - x.len() + 1 + i);
    }
  }
  Some(result)
}

fn bm_good_shift(x: &String) -> Vec<usize> {
  let mut out: Vec<usize> = Vec::with_capacity(x.len());
  for idx in (0..x.len()-1).rev() {
    let seg = &x[idx+1..];
    for seg_start in (1..(idx-seg.len())).rev() {
      let seg2 = &x[seg_start..seg.len()];
      if(seg == seg2) {
        out[idx] = seg_start - idx; 
        break;
      }
    }
    // if no match, align the longest suffix v of y
    if out[idx] == 0 {
      let mut max_ol = 0;
      for overlap_len in 1..seg.len()+1 {
        let seg1 = &x[0..overlap_len];
        let seg2 = &x[x.len()-overlap_len..x.len()];
        if seg1 == seg2 {
          max_ol = overlap_len;
        }
      }
      out[idx] = x.len() - max_ol;
    } // no match
  } // no matching char
}

fn bm_bad_shift(x: &String) -> HashMap<&str, usize> {
  let mut out: HashMap<&str, usize> = HashMap::new();
  for idx in 0..x.len() {
    out[x[idx]] = idx;
  }
  out
}

pub fn kmp() {
}
