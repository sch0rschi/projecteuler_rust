use num_bigint::BigInt;
use num_traits::One;
use std::ops::Add;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0025();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(4782, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0025() -> i32 {
    let mut f_p = BigInt::one();
    let mut f_n = BigInt::one();
    let mut n_th = 2;

    while f_n < BigInt::from(10).pow(999) {
        let temp = f_n.clone();
        f_n = f_n.add(f_p);
        f_p = temp;
        n_th += 1;
    }

    n_th
}