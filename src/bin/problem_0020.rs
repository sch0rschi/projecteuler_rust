use num_bigint::BigInt;
use projecteuler::evaluation_helper::solve_print_and_check;
use std::ops::{Add, Mul};

fn main() {
    solve_print_and_check(solve_0020, 648);
}
fn solve_0020() -> u32 {
    let mut factor = BigInt::from(1);
    let mut product = BigInt::from(1);

    while factor <= BigInt::from(100) {
        product = product.mul(factor.clone());
        factor = factor.add(1);
    }

    product
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum()

}
