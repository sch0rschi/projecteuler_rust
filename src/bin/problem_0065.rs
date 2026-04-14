use std::mem::swap;
use std::time::Instant;

use num_bigint::BigInt;

fn main() {
    let start = Instant::now();
    let result = solve_0065();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(272, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0065() -> u64 {
    let mut numerator = BigInt::from(1u64);
    let mut denominator = BigInt::from(0u64);

    for i in (1..=100).rev() {
        let a = match i % 3 {
            2 => 2 * (i / 3 + 1),
            _ => 1,
        };

        numerator += a * &denominator;
        swap(&mut numerator, &mut denominator);
    }

    numerator += BigInt::from(2u64) * &denominator;
    swap(&mut numerator, &mut denominator);

    denominator
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}
