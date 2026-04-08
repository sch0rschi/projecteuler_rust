use itertools::Itertools;
use projecteuler::primes::{primes_inclusive, Primes};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let Primes { prime_sieve: sieve, prime_list: primes_list } = primes_inclusive(999_999);

    let mut prefix_sums: Vec<i64> = Vec::with_capacity(primes_list.len());

    let mut prefix_sum: i64 = 0;
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
            if sieve[(end_sum - start_sum) as usize] && best_chain_count < end_index - start_index {
                best_chain_count = end_index - start_index;
                best_prime = end_sum - start_sum;
            }
        }
    }

    println!("{}", best_prime);
    println!("Elapsed: {:?}", start.elapsed());
}
