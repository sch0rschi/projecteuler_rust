use std::time::Instant;
use projecteuler::primes::Primes;

static LIMIT: usize = 100;

fn main() {
    let start = Instant::now();
    let result = solve_0077();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(71, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0077() -> u64 {
    let primes = Primes::primes_inclusive(LIMIT as u64);
    let primes: Vec<usize> = primes.primes_list.iter().map(|&p| p as usize).collect();

    let mut dp = vec![0u64; LIMIT + 1];
    dp[0] = 1;

    for &p in &primes {
        for i in p..=LIMIT {
            dp[i] += dp[i - p];
        }
        for (i, &item) in dp.iter().enumerate().take(p + 1).skip(1) {
            if item > 5_000 {
                return i as u64;
            }
        }
    }
    unreachable!()
}