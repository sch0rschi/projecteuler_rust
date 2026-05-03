use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::exponentiation::mod_pow;

const EXPONENT: u64 = 7_830_457;
const FACTOR: u64 = 28_433;
const MOD: u64 = 10_000_000_000;

fn main() {solve_print_and_check(solve_0097, 8739992577);
}

fn solve_0097() -> u64 {
    (FACTOR * mod_pow(2, EXPONENT, MOD) + 1) % MOD
}
