
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
  // To pass the test, either base ^ d = 1 (mod n),
  // or base ^ (d * 2^i) = n-1 (mod n), where 0 <= i < r
  let t = pow_mod(base, d, n);
  if t == 1 || t == (n-1) {
    return true;
  }
  for i in 1..r {
    let next_t = pow_mod(t, 2, n);
    if next_t == 1 || next_t == (n-1) {
      return true;
    }
  }
  false 
}
