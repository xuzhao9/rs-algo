//
// Permuation and Combination
// Author: xuzhao9 (i@@xuzhao.net)
//


#[cfg(test)]
mod test;

pub fn permutation(m: i32, n: i32) -> Option<Vec<Vec<i32>>> {
  if n == 0 { return None; }
  let mut results: Vec<Vec<i32>> = Vec::new();
  match combination(m, n) {
    None => {return None; }
    Some(x) => {
      for e in x {
        let p = gen_permutations(e);
        results.extend(p);
      }
    }
  }
  Some(results)
}

// Select a group of n elements from 1..m
pub fn combination(m: i32, n: i32) -> Option<Vec<Vec<i32>>> {
  if n <= 0 {
    return None;
  }
  let mut results: Vec<Vec<i32>> = Vec::new();
  for e in 1..m+1 {
    let mut h: Vec<i32> = Vec::new();
    h.push(e);
    results.push(h);
  }
  for num in 2..n+1 {
    let mut next_results: Vec<Vec<i32>> = Vec::new();
    for cur in results {
      let last = cur.last().unwrap();
      if last + num - (cur.len() as i32) > m + 1 { // not enough elements to select
        break;
      }
      for next in last+1..m+1 {
        let mut nn = cur.clone();
        nn.push(next);
        next_results.push(nn);
      }
    }
    results = next_results;
  }
  Some(results)
}

fn gen_permutations(input: Vec<i32>) -> Vec<Vec<i32>> {
  let mut results: Vec<Vec<i32>> = Vec::new();
  if input.len() == 0 {
    return results;
  }
  results.push(vec!(input[0]));
  if input.len() == 1 {
    return results;
  }
  for num in 2..(input.len()+1) {
    let to_insert = input[num - 1];
    let mut next_results: Vec<Vec<i32>> = Vec::new();
    for e in results {
      for index in 0..e.len()+1 {
        let mut t = e.clone();
        t.insert(index, to_insert);
        next_results.push(t);
      }
    }
    results = next_results;
  }
  results
}

