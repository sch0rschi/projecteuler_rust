use num_integer::Roots;
use projecteuler::coprimes::phi;
use projecteuler::digits::get_digit_count_encoding_15_max;
use std::time::Instant;
use projecteuler::primes::Primes;

fn main() {
    let start = Instant::now();
    let result = solve_0070();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(8319823, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0070() -> u64 {
    let limit = 10_000_000;
    let primes = Primes::primes_inclusive(limit.sqrt());

    (2..=limit)
        .filter_map(|n| {
            let factors = unique_prime_factors_2_capped(n, &primes)?;
            let phi = phi(n, &factors);
            if !is_permutation(n, phi) {
                return None;
            }
            let ratio = n * 1_000_000 / phi;
            Some((n, ratio))
        })
        .min_by_key(|&(_, ratio)| ratio)
        .unwrap()
        .0
}

fn is_permutation(p0: u64, p1: u64) -> bool {
    get_digit_count_encoding_15_max(p0) == get_digit_count_encoding_15_max(p1)
}

pub fn unique_prime_factors_2_capped(mut n: u64, primes: &Primes) -> Option<[u64; 2]> {
    let mut result = [0; 2];
    let mut count = 0;
    if n < 2 {
        return None;
    }
    for &p in &primes.primes_list {
        if p * p > n {
            break;
        }

        if n.is_multiple_of(p) {
            if count >= 2 {
                return None;
            }
            result[count] = p;
            count += 1;
            while n.is_multiple_of(p) {
                n /= p;
            }
        }
    }
    if n > 1 {
        if count != 1 {
            return None;
        }
        result[count] = n;
        count += 1;
    }

    if count != 2 {
        return None;
    }
    Some(result)
}
