use projecteuler::evaluation_helper::solve_print_and_check;
use std::fs;

fn main() {
    solve_print_and_check(solve_0099, 709);
}

fn solve_0099() -> usize {
    fs::read_to_string("resources/0099_base_exp.txt")
        .expect("Failed to read file")
        .lines()
        .enumerate()
        .map(|(line, x)| {
            let (base, exponent) = x.split_once(',').unwrap();
            let base: f64 = base.parse().unwrap();
            let exponent: f64 = exponent.parse().unwrap();

            (line, exponent * base.ln())
        })
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap()
        .0
        + 1
}
