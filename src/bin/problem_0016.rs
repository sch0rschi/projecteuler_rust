use num_bigint::BigUint;
use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0016, 1366);
}

fn solve_0016() -> u64 {
    let mut n = BigUint::from(2u64);
    n = n.pow(1000);

    let mut sum = 0u64;

    for byte in n.to_radix_be(10) {
        sum += byte as u64;
    }

    sum
}
