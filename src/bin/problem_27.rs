use projecteuler::primes::{primes_inclusive, Primes};
use std::collections::HashSet;

fn main() {
    let max_value_polynomial: i64 = 999i64.pow(2) + 999i64.pow(2) + 1000;
    let Primes { prime_sieve: _, prime_list } = primes_inclusive(max_value_polynomial as u64);
    let primes_set: HashSet<i64> = HashSet::from_iter(prime_list.iter().map(|&x| x as i64));

    let mut max_n = 0;
    let mut max_product = 0;
    for &b in prime_list.iter().take_while(|&p| *p <= 1000) {
        let b = b as i64;
        if b <= max_n {
            break;
        }
        for a in -999..=999 {
            let mut n: i64 = 0;
            loop {
                let polynomial_value = n * n + a * n + b;
                if primes_set.contains(&polynomial_value) {
                    n += 1;
                } else {
                    if n > max_n {
                        max_n = n;
                        max_product = a * b;
                    }
                    break;
                }
            }
        }
    }
    println!("{}", max_product);
}
