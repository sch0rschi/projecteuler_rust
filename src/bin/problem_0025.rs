use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0025, 4782);
}

// https://en.wikipedia.org/wiki/Fibonacci_sequence#Computation_by_rounding
// solve for F_n = 10^1000 by log both sides
fn solve_0025() -> i32 {
    let target_digits = 1000.0;

    let sqrt5 = 5f64.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;

    let n = ((target_digits - 1.0 + sqrt5.log10() / 2.0) / phi.log10()).ceil();

    n as i32
}
