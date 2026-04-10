use num_bigint::BigInt;
use std::collections::HashSet;
use std::ops::Mul;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0029();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(9183, result);
    assert!(duration < std::time::Duration::from_secs(1));
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
