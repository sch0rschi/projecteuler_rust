use itertools::Itertools;
use projecteuler::digits::{get_digits, get_number};
use projecteuler::primes::{primes_inclusive, Primes};
use std::time::Instant;

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
    let Primes { prime_sieve, prime_list } = primes_inclusive(1_000_000);
    println!("{}", prime_sieve[3797]);
    for &prime in prime_list.iter().dropping(4) {
        if count >= 11 {
            break;
        }
        if is_truncatable_prime(&prime_sieve, prime) {
            sum += prime;
            count += 1;
        }
    }
    sum
}

fn is_truncatable_prime(primes_set: &[bool], prime: u64) -> bool {
    let prime_digits = get_digits(prime);
    for i in 1..prime_digits.len() {
        let left_stripped = get_number(&prime_digits[i..prime_digits.len()]);
        if !primes_set[left_stripped as usize] {
            return false;
        }
        let right_stripped = get_number(&prime_digits[0..prime_digits.len() - i]);
        if !primes_set[right_stripped as usize] {
            return false;
        }
    }
    true
}
