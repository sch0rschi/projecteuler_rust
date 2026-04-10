use projecteuler::digits::{get_digits, get_digits_in_binary};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0036();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(872187, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0036() -> u64 {
    let mut sum = 0;
    for i in 1..1_000_000 {
        let digits = get_digits(i);
        if digits == digits.iter().rev().copied().collect::<Vec<u64>>() {
            let digits_in_binary = get_digits_in_binary(i);
            if digits_in_binary == digits_in_binary.iter().rev().copied().collect::<Vec<bool>>() {
                sum += i;
            }
        }
    }
    sum
}
