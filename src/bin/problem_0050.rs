use itertools::Itertools;
use projecteuler::primes::{primes_inclusive};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0050();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(997651, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0050() -> u64 {
    let primes = primes_inclusive(999_999);
    let primes_list = &primes.primes_list;

    let mut prefix_sums: Vec<u64> = Vec::with_capacity(primes_list.len());

    let mut prefix_sum: u64 = 0;
    for prime in primes_list {
        prefix_sum += prime;
        prefix_sums.push(prefix_sum);
    }

    let mut best_chain_count = 0;
    let mut best_prime = 0;

    for (end_index, end_sum) in prefix_sums.iter().enumerate().rev() {
        for (start_index, start_sum) in prefix_sums
            .iter()
            .enumerate()
            .rev()
            .dropping(prefix_sums.len() - end_index + best_chain_count)
        {
            if end_sum - start_sum >= 1_000_000 {
                break;
            }
            if primes.is_prime(end_sum - start_sum) && best_chain_count < end_index - start_index {
                best_chain_count = end_index - start_index;
                best_prime = end_sum - start_sum;
            }
        }
    }

    best_prime
}
