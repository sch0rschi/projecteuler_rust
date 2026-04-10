use projecteuler::digits::{get_digits, get_number};
use projecteuler::primes::{primes_inclusive, Primes};

fn main() {
    let Primes { prime_sieve, prime_list: primes } = primes_inclusive(1_000_000);
    let mut count = 1;
    for &prime in primes.iter().skip(1) {
        if is_rotating_prime(prime, &prime_sieve) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn is_rotating_prime(n: u64, primes: &[bool]) -> bool {
    let mut number_in_digits = get_digits(n as i64);
    if number_in_digits.contains(&0)
        || number_in_digits.contains(&2)
        || number_in_digits.contains(&4)
        || number_in_digits.contains(&6)
        || number_in_digits.contains(&8)
    {
        return false;
    }
    for _ in 0..number_in_digits.len() {
        let number = get_number(&number_in_digits);
        if !primes[number as usize] {
            return false;
        }
        number_in_digits.rotate_left(1);
    }
    true
}
