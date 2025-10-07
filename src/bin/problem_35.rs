use projecteuler::digits::{get_digits, get_number};
use projecteuler::primes::find_primes_up_to_inclusive;

fn main() {
    let primes = find_primes_up_to_inclusive(1_000_000);
    let mut count = 1;
    for &prime in primes.iter().skip(1) {
        if is_rotating_prime(prime, &primes) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn is_rotating_prime(n: i64, primes: &[i64]) -> bool {
    let mut number_in_digits = get_digits(n);
    if number_in_digits.contains(&0)
        || number_in_digits.contains(&2)
        || number_in_digits.contains(&4)
        || number_in_digits.contains(&6)
        || number_in_digits.contains(&8) {
        return false;
    }
    for _ in 0..number_in_digits.len() {
        let number = get_number(&number_in_digits);
        if !primes.contains(&number) {
            return false;
        }
        number_in_digits.rotate_left(1);
    }
    true
}
