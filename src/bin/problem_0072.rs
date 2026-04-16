use std::time::Instant;
use num_integer::{Roots};
use projecteuler::coprimes::phi;
use projecteuler::primes::primes_inclusive;

fn main() {
    let start = Instant::now();
    let result = solve_0072();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(303963552391, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0072() -> u64 {
    let limit = 1_000_000;
    let primes = primes_inclusive(limit.sqrt());

    (2..=limit)
        .map(|n| {
            let factors = primes.unique_prime_factors(n);
            phi(n, &factors)
        })
        .sum()
}
