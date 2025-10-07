use std::collections::HashSet;
use std::ops::Mul;
use num_bigint::BigInt;

fn main() {
    let mut result_set : HashSet<BigInt> = HashSet::new();
    for a in 2..=100 {
        let big_int_a = BigInt::from(a);
        let mut big_int_power = big_int_a.clone();
        for _ in 2..=100 {
            big_int_power = big_int_power.mul(big_int_a.clone());
            result_set.insert(big_int_power.clone());
        }
    }
    println!("{}", result_set.len());
}
