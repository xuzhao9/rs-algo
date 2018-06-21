//
// Longest substring problems
//

// Longest common substring
pub fn common_substring(x: String, y: String) -> String {
  let mut suffix:Vec<Vec<i32>> = vec![ [0; x.len()]; y.len()];
  let mut max_i = 0;
  let mut max_j = 0;
  let mut max_suffix = 0;
  for i in 0..x.len() {
    for j in 0..y.len() {
      if x[i] == y[j] {
        if i == 0 || j == 0 {
          suffix[i][j] = 1;
        } else {
          suffix[i][j] = suffix[i-1][j-1] + 1;
          if suffix[i][j] > max_suffix {
            max_suffix = suffix[i][j];
            max_i = i;
            max_j = j;
          }
        }
      } else {
        suffix[i][j] = 0;
      }
    }
  }
  let mut len = 0;
  loop {
    if x[max_i + len] == y[max_j + len] {
      len += 1;
    } else {
      break;
    }
  }
  String::from(x[max_i..max_i+len])
}

// Longest non-descending substring
pub fn nondescend_substring(x: String) -> String {
  String::new()
}
