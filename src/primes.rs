pub struct Primes {
    pub prime_sieve: Vec<bool>,
    pub prime_list: Vec<u64>,
}

pub fn find_first_n_primes(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = [2, 3].to_vec();

    for _ in 3..=n {
        let next_prime = find_next_prime(&primes);
        primes.push(next_prime);
    }
    primes
}

pub fn primes_inclusive(limit: u64) -> Primes {
    let sieve = prime_sieve_up_to_inclusive(limit as usize);
    let list = sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i as u64) } else { None })
        .collect();
    Primes {
        prime_sieve: sieve.clone(),
        prime_list: list,
    }
}

pub fn prime_sieve_up_to_inclusive(limit: usize) -> Vec<bool> {
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

fn find_next_prime(previous_primes: &[u64]) -> u64 {
    let last_prime = *previous_primes.last().unwrap();
    for candidate_prime in ((last_prime + 2)..).step_by(2) {
        let candidate_prime_sqrt = candidate_prime.isqrt();

        if previous_primes
            .iter()
            .take_while(|&&p| p <= candidate_prime_sqrt)
            .all(|&previous_prime| candidate_prime % previous_prime != 0)
        {
            return candidate_prime;
        }
    }
    unreachable!("The loop should always return a prime");
}

pub fn add_next_prime(previous_primes: &mut Vec<u64>) {
    let next_prime = find_next_prime(previous_primes);
    previous_primes.push(next_prime);
}

pub fn unique_prime_factors(n: u64, list: &[u64]) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    let mut number = n;
    for &prime in list.iter() {
        if number.is_multiple_of(prime) {
            number /= prime;
            factors.push(prime);
            while number.is_multiple_of(prime) {
                number /= prime;
            }
        }

        if number == 1 {
            break;
        }
    }

    factors
}
