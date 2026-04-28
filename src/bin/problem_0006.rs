use projecteuler::evaluation_helper::solve_print_and_check;

const LIMIT: u32 = 100u32;

fn main() {
    solve_print_and_check(solve_0006, 25164150);
}

fn solve_0006() -> u32 {
    let sum = LIMIT * (LIMIT + 1) / 2;
    let sum_sq = LIMIT * (LIMIT + 1) * (2 * LIMIT + 1) / 6;
    sum * sum - sum_sq
}
