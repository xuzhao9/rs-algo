//! Prime number related functions
//! - Rabin-Miller Primality Test

// Rabin-Miller primality test
// When the input number is i64 (< 10^19),
// base array: [2, 3, 7, 61, 24251]
pub fn is_prime_rabin_miller(number: i64) -> bool {
  let base = [2, 3, 7, 61, 24251];
  if number <= 1 {
    return false;
  }
  if number % 2 == 0 && number != 2 {
    return false;
  }
  for b in base.iter().cloned() {
    if b >= number {
      return true;
    }
    if !is_prime_rabin_miller_base(number, b) {
      return false;
    }
  }
  true
}

// Return base^power (mod mbase) using fast power algorithm
fn pow_mod(base: i64, power: i64, mbase: i64) -> i64 {
  match power {
    0 => 1,
    1 => (base % mbase),
    x => {
      if x & 1 == 0 {
        return pow_mod((base * base) % mbase, power / 2, mbase) % mbase;
      } else {
        return pow_mod((base * base) % mbase, power / 2, mbase) * base % mbase;
      }
    },
  }
}

fn is_prime_rabin_miller_base(n: i64, base: i64) -> bool {
  let power = n - 1;
  let mut d = power;
  let mut r = 0;
  while d & 1 == 0 {
    d = d >> 1;
    r += 1;
  }
  // n-1 = d * (2 ^ r)
  // n is a probable prime, either base ^ d = +1/-1 (mod n),
  // or base ^ (d * 2^i) = -1 (mod n), where 0 < i <= r-1
  // When 0 < i <= r-1, if base ^ (d * 2 ^ i) = 1 (mod n), n must be composite
  // Because 1 has a square root other than +1/-1
  let t = pow_mod(base, d, n);
  if t == 1 || t == (n-1) {
    return true;
  }
  for _i in 1..r {
    let next_t = pow_mod(t, 2, n);
    if next_t == 1 {
      return false;
    }
    if next_t == (n-1) {
      return true;
    }
  }
  false 
}
