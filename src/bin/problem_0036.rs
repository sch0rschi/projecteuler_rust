use projecteuler::digits::{get_digits, get_digits_in_binary};
use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0036, 872187);
}

fn solve_0036() -> u64 {
    let mut sum = 0;
    for i in 1..1_000_000 {
        let digits = get_digits(i);
        if digits == digits.iter().rev().copied().collect::<Vec<u64>>() {
            let digits_in_binary = get_digits_in_binary(i);
            if digits_in_binary
                == digits_in_binary
                    .iter()
                    .rev()
                    .copied()
                    .collect::<Vec<bool>>()
            {
                sum += i;
            }
        }
    }
    sum
}
