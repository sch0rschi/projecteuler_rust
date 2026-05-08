use projecteuler::evaluation_helper::solve_print_and_check;
use std::iter::Iterator;

const FACTORIAL: [u64; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
const NINE_FACTORIAL: u64 = FACTORIAL[9];

fn main() {
    solve_print_and_check(solve_0034, 40730);
}

fn solve_0034() -> u64 {
    (2u32..)
        .take_while(|&n| n as u64 * NINE_FACTORIAL >= 10u64.pow(n - 1))
        .map(sum_for_number_length)
        .sum()
}

fn sum_for_number_length(number_of_digits: u32) -> u64 {
    let min = 10u64.pow(number_of_digits - 1);
    let max = min * 10 - 1;
    // min digit is 1 since 0! == 1! leads to double counting otherwise
    sum_for_number_length_recursion(1, 0, number_of_digits, min, max)
}

fn sum_for_number_length_recursion(
    min_digit: usize,
    set_digits_sum: u64,
    remaining: u32,
    min: u64,
    max: u64,
) -> u64 {
    if remaining == 0 {
        return if is_digit_factorial_sum(set_digits_sum) {
            set_digits_sum
        } else {
            0
        };
    }

    FACTORIAL
        .iter()
        .enumerate()
        .filter(|&(digit, &fact)| {
            digit >= min_digit
                && set_digits_sum + fact + (remaining - 1) as u64 * NINE_FACTORIAL >= min
                && set_digits_sum + fact <= max
        })
        .map(|(digit, &fact)| {
            sum_for_number_length_recursion(
                digit,
                set_digits_sum + fact,
                remaining - 1,
                min,
                max,
            )
        })
        .sum()
}

fn is_digit_factorial_sum(number: u64) -> bool {
    let mut digit_sum = 0;
    let mut n = number;

    while n > 0 {
        digit_sum += FACTORIAL[(n % 10) as usize];
        n /= 10;
    }

    number == digit_sum
}
