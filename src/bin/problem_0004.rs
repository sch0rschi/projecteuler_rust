use num_integer::Roots;
use projecteuler::evaluation_helper::solve_print_and_check;
use std::ops::RangeInclusive;

fn main() {
    solve_print_and_check(solve_0004, 906609);
}

fn solve_0004() -> u32 {
    RangeInclusive::new(100, 999)
        .rev()
        .map(make_palindrome)
        .find(|&p| is_valid_product(p))
        .unwrap()
}

fn is_valid_product(palindrome: u32) -> bool {
    let lower = (palindrome / 999).max(palindrome.sqrt());
    let upper = (palindrome / 100).min(999);
    for divisor in lower..upper + 1 {
        if palindrome.is_multiple_of(divisor) {
            return true;
        }
    }
    false
}

fn make_palindrome(mut n: u32) -> u32 {
    let mut accumulator = n;

    while n > 0 {
        accumulator = accumulator * 10 + n % 10;
        n /= 10;
    }

    accumulator
}
