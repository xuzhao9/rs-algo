use super::prime::*;

#[test]
fn test_is_prime() {
  let numbers = [983, 427, 719, 9167, 8491];
  let answers = [true, false, true, false, false];
  let results: Vec<_> = numbers.iter().map(|n| is_prime_rabin_miller(*n)).collect();
  assert_eq!(results, answers);
}
