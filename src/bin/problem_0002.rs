use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0002, 4613732);
}

const LIMIT: u32 = 4_000_000;

fn solve_0002() -> u32 {
    let mut f_n = 2;
    let mut f_n3 = 8;
    let mut sum = 2;

    while f_n3 <= LIMIT {
        sum += f_n3;
        (f_n, f_n3) = (f_n3, 4 * f_n3 + f_n);
    }

    sum
}
