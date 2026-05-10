use crate::libs::exponentiation::mod_pow;

const SMALL_PRIMES: &[u64] = &[7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

pub struct Primes {
    pub primes_list: Vec<u64>,
    limit: usize,
    is_prime: Vec<bool>,
}

impl Primes {
    pub fn primes_inclusive(limit: u64) -> Self {
        let limit = limit as usize;

        if limit < 2 {
            return Self {
                primes_list: Vec::new(),
                limit,
                is_prime: vec![],
            };
        }

        let size = (limit >> 1) + 1;
        let mut is_prime = vec![true; size];

        is_prime[0] = false;

        let mut primes_list = Vec::with_capacity((limit as f64 / (limit as f64).ln()) as usize);

        primes_list.push(2);

        let mut p = 3;

        while p * p <= limit {
            let idx = p >> 1;

            if is_prime[idx] {
                let mut m = p * p;
                let step = 2 * p;

                while m <= limit {
                    is_prime[m >> 1] = false;
                    m += step;
                }

                primes_list.push(p as u64);
            }

            p += 2;
        }

        while p <= limit {
            if is_prime[p >> 1] {
                primes_list.push(p as u64);
            }
            p += 2;
        }

        Self {
            primes_list,
            limit,
            is_prime,
        }
    }

    #[inline(always)]
    pub fn is_prime(&self, n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n.is_multiple_of(2) {
            return false;
        }

        let n_usize = n as usize;

        if n_usize <= self.limit {
            return self.is_prime[n_usize >> 1];
        }

        is_prime(n)
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
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 {
        return true;
    }
    if n.is_multiple_of(2) || n.is_multiple_of(3) || n.is_multiple_of(5) {
        return false;
    }

    if n < 100_000_000 {
        return is_prime_wheel_30(n);
    }

    for &p in SMALL_PRIMES {
        if n == p {
            return true;
        }
        if n.is_multiple_of(p) {
            return false;
        }
    }

    miller_rabin(n)
}

pub fn is_prime_wheel_30(n: u64) -> bool {
    const CYCLE: [u64; 8] = [4, 2, 4, 2, 4, 6, 2, 6];

    let mut divisor = 7u64;
    let mut i = 0usize;

    while divisor * divisor <= n {
        if n.is_multiple_of(divisor) {
            return false;
        }

        divisor += CYCLE[i];
        i = (i + 1) & 7;
    }

    true
}


fn miller_rabin(n: u64) -> bool {
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

    panic!("Number too large for Miller-Rabin test: {}", n);
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
    let mut is_composite = x != 1 && x != n - 1;

    for _ in 1..s {
        x = mod_mul(x, x, n);
        is_composite &= x != n - 1;
    }

    !is_composite
}

fn is_prime_miller_rabin(n: u64, bases: &[u64]) -> bool {
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
