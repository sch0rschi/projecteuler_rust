use num_bigint::BigInt;
use std::ops::{Add, Mul};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0020();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(648, result);
    assert!(duration < std::time::Duration::from_secs(1));
}
fn solve_0020() -> u32 {
    let mut factor = BigInt::from(1);
    let mut product = BigInt::from(1);

    while factor <= BigInt::from(100) {
        product = product.mul(factor.clone());
        factor = factor.add(1);
    }

    product.to_string().chars().map(|x| x.to_digit(10).unwrap()).sum::<u32>()
}