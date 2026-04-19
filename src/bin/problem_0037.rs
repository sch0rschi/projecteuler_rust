use itertools::Itertools;
use projecteuler::digits::{get_digits, get_number};
use std::time::Instant;
use projecteuler::primes::Primes;

fn main() {
    let start = Instant::now();
    let result = solve_0037();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(748317, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0037() -> u64 {
    let mut count = 0;
    let mut sum = 0;
    let primes = Primes::primes_inclusive(1_000_000);
    let primes_list = &primes.primes_list;
    for &prime in primes_list.iter().dropping(4) {
        if count >= 11 {
            break;
        }
        if is_truncatable_prime(&primes, prime) {
            sum += prime;
            count += 1;
        }
    }
    sum
}

fn is_truncatable_prime(primes: &Primes, prime: u64) -> bool {
    let prime_digits = get_digits(prime);
    for i in 1..prime_digits.len() {
        let left_stripped = get_number(&prime_digits[i..prime_digits.len()]);
        if !primes.is_prime(left_stripped) {
            return false;
        }
        let right_stripped = get_number(&prime_digits[0..prime_digits.len() - i]);
        if !primes.is_prime(right_stripped) {
            return false;
        }
    }
    true
}
