use projecteuler::primes;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0007();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(104743, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0007() -> u64 {
    let primes = primes::find_first_n_primes(10001);
    *primes.last().unwrap()
}
