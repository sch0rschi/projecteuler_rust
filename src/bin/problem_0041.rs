use projecteuler::pandigital::is_1_to_length_pandigital;
use projecteuler::primes::{primes_inclusive, Primes};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0041();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(7652413, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0041() -> u64 {
    let Primes {
        prime_sieve: _,
        prime_list,
    } = primes_inclusive(7654321);

    *prime_list
        .iter()
        .rev()
        .find(|&&p| is_1_to_length_pandigital(p))
        .unwrap()
}
