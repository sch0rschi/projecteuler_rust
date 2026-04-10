use num_bigint::BigInt;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0016();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(1366, result);
    assert!(duration < std::time::Duration::from_secs(1));
}
fn solve_0016() -> i64 {
    let base = BigInt::from(2);
    let number = base.pow(1000);
    number.to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).sum::<i64>()
}