use projecteuler::primes::{primes_inclusive, Primes};

fn main() {
    let Primes {sieve: _, list: prime_list } = primes_inclusive(2_000_000);
    println!("{}", prime_list.iter().sum::<i64>());
}
