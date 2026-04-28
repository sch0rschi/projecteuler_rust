use std::time::Instant;
use projecteuler::exponentiation::mod_pow;

const EXPONENT: usize = 7_830_457;
const FACTOR: u64 = 28_433;
const MOD: u64 = 10_000_000_000;

fn main() {
    let start = Instant::now();
    let result = solve_0097();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(8739992577, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0097() -> u64 {
    (FACTOR * mod_pow(2, EXPONENT, MOD) + 1) % MOD
}
