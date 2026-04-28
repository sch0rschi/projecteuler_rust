use num_bigint::BigInt;
use projecteuler::evaluation_helper::solve_print_and_check;
use std::collections::HashSet;
use std::ops::Mul;

fn main() {
    solve_print_and_check(solve_0029, 9183);
}

fn solve_0029() -> usize {
    let mut result_set: HashSet<BigInt> = HashSet::new();
    for a in 2..=100 {
        let big_int_a = BigInt::from(a);
        let mut big_int_power = big_int_a.clone();
        for _ in 2..=100 {
            big_int_power = big_int_power.mul(big_int_a.clone());
            result_set.insert(big_int_power.clone());
        }
    }
    result_set.len()
}
