use crate::libs::exponentiation::mod_pow_u32;

const SMALL_PRIMES: &[u32] = &[7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

pub fn is_prime(n: u32) -> bool {
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

pub fn is_prime_wheel_30(n: u32) -> bool {
    const CYCLE: [u32; 8] = [4, 2, 4, 2, 4, 6, 2, 6];

    let mut divisor = 7u32;
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


fn miller_rabin(n: u32) -> bool {
    if n < 2_047 {
        return is_prime_miller_rabin(n, &[2]);
    }
    if n < 1_373_653 {
        return is_prime_miller_rabin(n, &[2, 3]);
    }
    if n < 9_080_191 {
        return is_prime_miller_rabin(n, &[31, 73]);
    }
    is_prime_miller_rabin(n, &[2, 7, 61])
}

#[inline(always)]
fn mod_mul(a: u32, b: u32, modulo: u32) -> u32 {
    if modulo < (1 << 16) {
        (a * b) % modulo
    } else {
        ((a as u64 * b as u64) % modulo as u64) as u32
    }
}

#[inline(always)]
fn miller_rabin_witness(n: u32, a: u32, d: u32, s: u32) -> bool {
    let mut x = mod_pow_u32(a, d, n);
    let mut is_composite = x != 1 && x != n - 1;

    for _ in 1..s {
        x = mod_mul(x, x, n);
        is_composite &= x != n - 1;
    }

    !is_composite
}

fn is_prime_miller_rabin(n: u32, bases: &[u32]) -> bool {
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
