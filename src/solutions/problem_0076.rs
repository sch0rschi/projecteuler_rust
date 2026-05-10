const LIMIT: usize = 100;


pub fn solve_0076() -> u64 {
    let mut dp = vec![0u64; LIMIT + 1];
    dp[0] = 1;

    for i in 1..LIMIT {
        for j in i..=LIMIT {
            dp[j] += dp[j - i];
        }
    }

    dp[LIMIT]
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0076::solve_0076;

    #[test]
    fn test() {
        solve_print_and_check(solve_0076, 190569291);
    }
}
