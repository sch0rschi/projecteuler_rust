use std::ops::{Add, Mul};
use num_bigint::BigInt;

fn main() {
    let mut factor = BigInt::from(1);
    let mut product = BigInt::from(1);

    while factor <= BigInt::from(100) {
        product = product.mul(factor.clone());
        factor = factor.add(1);
    }

    println!("{}", product.to_string().chars().map(|x| x.to_digit(10).unwrap()).sum::<u32>());
}