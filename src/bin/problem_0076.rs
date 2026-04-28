use projecteuler::evaluation_helper::solve_print_and_check;

const LIMIT: usize = 100;
fn main() {
    solve_print_and_check(solve_0076, 190569291);
}

fn solve_0076() -> u64 {
    let mut dp = vec![0u64; LIMIT + 1];
    dp[0] = 1;

    for i in 1..LIMIT {
        for j in i..=LIMIT {
            dp[j] += dp[j - i];
        }
    }

    dp[LIMIT]
}
