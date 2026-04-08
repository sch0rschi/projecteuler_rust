use projecteuler::digits::{get_digits, get_number};
use projecteuler::primes::{add_next_prime, primes_inclusive, Primes};
use std::collections::HashSet;

fn main() {
    let mut count = 0;
    let mut sum = 0;
    let Primes { prime_sieve: _, prime_list: mut primes } = primes_inclusive(7);
    let mut primes_set: HashSet<i64> = primes.iter().cloned().collect();
    let mut prime_index = 4;
    while count < 11 {
        add_next_prime(&mut primes);
        primes_set.insert(*primes.last().unwrap());
        let prime = primes[prime_index];
        prime_index += 1;
        if is_truncatable_prime(&mut primes_set, prime) {
            sum += prime;
            count += 1;
        }
    }
    println!("{}", sum);
}

fn is_truncatable_prime(primes_set: &mut HashSet<i64>, prime: i64) -> bool {
    let prime_digits = get_digits(prime);
    for i in 1..prime_digits.len() {
        let left_stripped = get_number(&prime_digits[i..prime_digits.len()]);
        if !primes_set.contains(&left_stripped) {
            return false;
        }
        let right_stripped = get_number(&prime_digits[0..prime_digits.len() - i]);
        if !primes_set.contains(&right_stripped) {
            return false;
        }
    }
    true
}
