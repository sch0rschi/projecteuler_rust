use projecteuler::evaluation_helper::solve_print_and_check;

const LIMIT: u64 = 504000;
fn main() {
    solve_print_and_check(solve_0000, 21337343999916000);
}

fn solve_0000() -> u64 {
    let m: u64 = LIMIT / 2;
    m * (2*m - 1) * (2*m + 1) / 3
}
