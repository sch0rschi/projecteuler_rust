use std::time::Instant;
use projecteuler::primes::Primes;

fn main() {
    let start = Instant::now();
    let result = solve_0010();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(142913828922, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0010() -> u64 {
    let primes = Primes::primes_inclusive(2_000_000);
    primes.primes_list.iter().sum::<u64>()
}
