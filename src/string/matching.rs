//
// String matching algorithms
// Author: xuzhao9 (i@xuzhao.net)
//

use std::collections::HashMap;
use std::cmp;

// If no match, return none
// Otherwise, return Some(index), where index is the first match
pub fn boyer_moore(y: String, x: String) -> Option<Vec<usize>> {
  let mut j: usize = 0;
  let mut i: usize = 0;
  if y.len() < x.len() {
    return None;
  }
  let gs = bm_good_shift(&x);
  let bs = bm_bad_shift(&x);
    
  let mut result: Vec<usize> = Vec::new();
  while j <= y.len() - x.len() {
    let mut mismatch = false;
    let mut mismatch_ind = 0;
    for i in (0..x.len()).rev() {
      if x[i..i+1] != y[i+j..(i+j+1)] {
        mismatch = true;
        mismatch_ind = i;
        break;
      }
    }
    if !mismatch { // find a full match
      result.push(j);
      j += gs[0];
    } else {
      i = mismatch_ind;
      let ys = &y[i+j..i+j+1];
      if bs.contains_key(ys) {
        j += cmp::max(gs[i], i - bs[ys]);
      } else {
        j += cmp::max(gs[i], i + 1);
      }
    }
  }
  Some(result)
}

fn bm_good_shift(x: &String) -> Vec<usize> {
  let mut out: Vec<usize> = Vec::with_capacity(x.len());
  for i in 0..x.len() {
    out.push(0);
  }
  for idx in (0..x.len()).rev() {
    if idx + 1 == x.len() {
      out[idx] = 1;
      continue;
    }
    let seg = &x[idx+1..];
    println!("idx = {} , seg = {}", idx, seg);
    if idx >= seg.len()+1 {
      for seg_start in (1..(idx-seg.len())).rev() {
        let seg2 = &x[seg_start..seg.len()];
        if(seg == seg2) {
          out[idx] = seg_start - idx; 
          break;
        }
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
  out
}

fn bm_bad_shift(x: &String) -> HashMap<&str, usize> {
  let mut out: HashMap<&str, usize> = HashMap::new();
  for idx in 0..x.len() {
    out.insert(&x[idx..idx+1], idx);
  }
  out
}

pub fn kmp() {
}
