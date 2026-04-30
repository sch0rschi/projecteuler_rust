use projecteuler::evaluation_helper::solve_print_and_check;

const N: u64 = 20;

fn main() {
    solve_print_and_check(solve_0015, 137846528820);
}

fn solve_0015() -> u64 {
    let mut result: u64 = 1;

    // How many permutations of (R,R,R,R,...,D,D,D,D,...) are there?
    // Safe binomial computation
    for i in 1..=N {
        result = result * (N + i) / (i);
    }

    result
}
