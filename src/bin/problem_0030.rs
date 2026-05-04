use projecteuler::evaluation_helper::solve_print_and_check;
use std::iter::Iterator;

const DIGIT_POW_5: [u32; 10] = [0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049];
const NINE_POW_FIVE: u32 = DIGIT_POW_5[9];

fn main() {
    solve_print_and_check(solve_0030, 443839);
}

fn solve_0030() -> u32 {
    (2u32..)
        .take_while(|&n| n * NINE_POW_FIVE >= 10u32.pow(n - 1))
        .map(sum_for_number_length)
        .sum()
}

fn sum_for_number_length(number_of_digits: u32) -> u32 {
    let min = 10u32.pow(number_of_digits - 1);
    let max = min * 10 - 1;
    sum_for_number_length_recursion(0, 0, number_of_digits, min, max)
}

fn sum_for_number_length_recursion(
    min_digit: usize,
    set_digits_sum: u32,
    remaining: u32,
    min: u32,
    max: u32,
) -> u32 {
    if remaining == 0 {
        return if are_digits_in_number(set_digits_sum) {
            set_digits_sum
        } else {
            0
        };
    }
    DIGIT_POW_5
        .iter()
        .enumerate()
        .filter(|&(digit, &pow_5)| {
            digit >= min_digit
                && set_digits_sum + pow_5 + (remaining - 1) * NINE_POW_FIVE >= min
                && set_digits_sum + pow_5 <= max
        })
        .map(|(digit, &pow_5)| {
            sum_for_number_length_recursion(digit, set_digits_sum + pow_5, remaining - 1, min, max)
        })
        .sum()
}

fn are_digits_in_number(number: u32) -> bool {
    let mut digit_power_sum = 0;
    let mut n = number;
    while n > 0 {
        digit_power_sum += DIGIT_POW_5[(n % 10) as usize];
        n /= 10;
    }
    number == digit_power_sum
}
