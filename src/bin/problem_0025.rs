use num_bigint::BigInt;
use num_traits::One;
use projecteuler::evaluation_helper::solve_print_and_check;
use std::ops::Add;

fn main() {
    solve_print_and_check(solve_0025, 4782);
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
