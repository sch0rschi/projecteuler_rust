use std::time::Instant;

const SINGLE_DIGIT_LENGTHS: [u64; 10] = [
    0, // 0 unused
    3, // one
    3, // two
    5, // three
    4, // four
    4, // five
    3, // six
    5, // seven
    5, // eight
    4, // nine
];

const TEEN_LENGTHS: [u64; 10] = [
    3, // ten
    6, // eleven
    6, // twelve
    8, // thirteen
    8, // fourteen
    7, // fifteen
    7, // sixteen
    9, // seventeen
    8, // eighteen
    8, // nineteen
];

const TENS_LENGTHS: [u64; 10] = [
    0, 0, 6, // twenty
    6, // thirty
    5, // forty
    5, // fifty
    5, // sixty
    7, // seventy
    6, // eighty
    6, // ninety
];

const HUNDRED_LENGTH: u64 = 7; // "hundred"
const AND_LENGTH: u64 = 3; // "and"
const THOUSAND_LENGTH: u64 = 11; // "one thousand"

fn main() {
    let start = Instant::now();
    let result = solve_0017();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(21124, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0017() -> u64 {
    // ===== 1–99 =====
    // 1–9 appear 10 times each
    let sum_small = SINGLE_DIGIT_LENGTHS.iter().sum::<u64>() * 9;
    // 10–19 appear once each
    let sum_teens = TEEN_LENGTHS.iter().sum::<u64>();
    // 20–90 appear 10 times each
    let sum_tens = TENS_LENGTHS.iter().sum::<u64>() * 10;
    let sum_1_to_99 = sum_small + sum_teens + sum_tens;

    // ===== 100–999 =====
    // "one hundred", ..., "nine hundred"
    let sum_hundreds = SINGLE_DIGIT_LENGTHS.iter().sum::<u64>() * 100 + 9 * 100 * HUNDRED_LENGTH;
    // "and" appears in 9 * 99 numbers
    let sum_and = 9 * 99 * AND_LENGTH;
    // 1–99 repeats 9 times
    let sum_repeated = sum_1_to_99 * 9;
    let sum_100_to_999 = sum_hundreds + sum_and + sum_repeated;

    // ===== total =====
    sum_1_to_99 + sum_100_to_999 + THOUSAND_LENGTH
}
