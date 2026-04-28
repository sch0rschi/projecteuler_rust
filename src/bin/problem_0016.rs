use num_bigint::BigInt;
use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0016, 1366);
}
fn solve_0016() -> i64 {
    let base = BigInt::from(2);
    let number = base.pow(1000);
    number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .sum()

}
