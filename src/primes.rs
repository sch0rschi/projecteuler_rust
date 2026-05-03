use crate::exponentiation::mod_pow;

pub struct Primes {
    prime_sieve: Vec<bool>,
    pub primes_list: Vec<u64>,
}

impl Primes {
    pub fn primes_inclusive(limit: u64) -> Self {
        let sieve = prime_sieve_up_to_inclusive(limit as usize);
        let list = sieve
            .iter()
            .enumerate()
            .filter_map(|(i, &is_prime)| if is_prime { Some(i as u64) } else { None })
            .collect();
        Primes {
            prime_sieve: sieve,
            primes_list: list,
        }
    }

    pub fn unique_prime_factors(&self, mut n: u64) -> Vec<u64> {
        let mut result = Vec::new();
        if n < 2 {
            return result;
        }
        for &p in &self.primes_list {
            if p * p > n {
                break;
            }

            if n.is_multiple_of(p) {
                result.push(p);
                while n.is_multiple_of(p) {
                    n /= p;
                }
            }
        }
        if n > 1 {
            result.push(n);
        }

        result
    }

    pub fn prime_factors(&self, mut n: u64) -> Vec<u64> {
        let mut result = Vec::new();
        if n < 2 {
            return result;
        }
        for &p in &self.primes_list {
            if n == 1 {
                break;
            }

            while n.is_multiple_of(p) {
                result.push(p);
                n /= p;
            }
        }

        result
    }

    pub fn is_prime(&self, n: u64) -> bool {
        if n < self.prime_sieve.len() as u64 {
            return self.prime_sieve[n as usize];
        }
        if n < 2 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        if n.is_multiple_of(2) || n.is_multiple_of(3) {
            return false;
        }
        if n < 2_047 {
            return is_prime_miller_rabin(n, &[2]);
        }
        if n < 1_373_653 {
            return is_prime_miller_rabin(n, &[2, 3]);
        }
        if n < 9_080_191 {
            return is_prime_miller_rabin(n, &[31, 73]);
        }
        if n < 4_759_123_141 {
            return is_prime_miller_rabin(n, &[2, 7, 61]);
        }
        if n < 2_152_302_898_747 {
            return is_prime_miller_rabin(n, &[2, 3, 5, 7, 11]);
        }
        if n < 3_474_749_660_383 {
            return is_prime_miller_rabin(n, &[2, 3, 5, 7, 11, 13]);
        }
        if n < 341_550_071_728_321 {
            return is_prime_miller_rabin(n, &[2, 3, 5, 7, 11, 13, 17]);
        }
        panic!("This code path should not be reached");
    }
}

fn prime_sieve_up_to_inclusive(limit: usize) -> Vec<bool> {
    if limit < 2 {
        return vec![false; limit + 1];
    }

    let size = (limit >> 1) + 1;
    let mut bits = vec![u64::MAX; size.div_ceil(64)];

    bits[0] &= !1u64;

    let mut p = 3;

    while p * p <= limit {
        let pi = p >> 1;

        if ((bits[pi >> 6] >> (pi & 63)) & 1) == 1 {
            let mut m = p * p;

            while m <= limit {
                let mi = m >> 1;
                bits[mi >> 6] &= !(1u64 << (mi & 63));
                m += 2 * p;
            }
        }

        p += 2;
    }

    let mut out = vec![false; limit + 1];

    if limit >= 2 {
        out[2] = true;
    }

    let mut i = 3;
    while i <= limit {
        let idx = i >> 1;
        let bit = (bits[idx >> 6] >> (idx & 63)) & 1;
        out[i] = bit == 1;
        i += 2;
    }

    out
}

#[inline(always)]
fn mod_mul(a: u64, b: u64, modulo: u64) -> u64 {
    if modulo < (1 << 32) {
        (a * b) % modulo
    } else {
        ((a as u128 * b as u128) % modulo as u128) as u64
    }
}

#[inline(always)]
fn miller_rabin_witness(n: u64, a: u64, d: u64, s: u32) -> bool {
    let mut x = mod_pow(a, d, n);

    if x == 1 || x == n - 1 {
        return true;
    }

    for _ in 1..s {
        x = mod_mul(x, x, n);
        if x == n - 1 {
            return true;
        }
    }

    false
}

fn is_prime_miller_rabin(n: u64, bases: &[u64]) -> bool {
    const SMALL_PRIMES: &[u64] = &[5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];
    for &p in SMALL_PRIMES {
        if n == p {
            return true;
        }
        if n.is_multiple_of(p) {
            return false;
        }
    }

    let mut d = n - 1;
    let s = d.trailing_zeros();
    d >>= s;

    for &a in bases {
        if a >= n && a % n == 0 {
            continue;
        }
        if !miller_rabin_witness(n, a, d, s) {
            return false;
        }
    }

    true
}
