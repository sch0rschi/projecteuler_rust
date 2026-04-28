use projecteuler::digits::{get_digits, get_number};
use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::primes::Primes;

fn main() {
    solve_print_and_check(solve_0035, 55);
}

fn solve_0035() -> i32 {
    let primes = Primes::primes_inclusive(1_000_000);
    let primes_list = &primes.primes_list;
    let mut count = 1;
    for &prime in primes_list.iter().skip(1) {
        if is_rotating_prime(prime, &primes) {
            count += 1;
        }
    }
    count
}

fn is_rotating_prime(n: u64, primes: &Primes) -> bool {
    let mut number_in_digits = get_digits(n);
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
        if !primes.is_prime(number) {
            return false;
        }
        number_in_digits.rotate_left(1);
    }
    true
}
