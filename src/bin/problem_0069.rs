use projecteuler::primes::primes_inclusive;
use std::time::Instant;
use projecteuler::coprimes::phi;

fn main() {
    let start = Instant::now();
    let result = solve_0069();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(510510, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0069() -> usize {
    let primes = primes_inclusive(1_000);

    (2..=1_000_000)
        .map(|n| {
            let factors = primes.unique_prime_factors(n as u64);
            let phi = phi(n as u64, &factors);
            (n, n as u64 * 1_000_000u64 / phi)
        })
        .max_by_key(|&(_, ratio)| ratio)
        .unwrap()
        .0
}
