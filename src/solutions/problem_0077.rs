use crate::libs::primes::Primes;

const LIMIT: usize = 100;


pub fn solve_0077() -> usize {
    let primes = Primes::primes_inclusive(LIMIT);

    let mut dp = vec![0usize; LIMIT + 1];
    dp[0] = 1;

    for p in primes.single_iterator() {
        for i in p..=LIMIT {
            dp[i] += dp[i - p];
        }
        for (i, &item) in dp.iter().enumerate().take(p + 1).skip(1) {
            if item > 5_000 {
                return i;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0077::solve_0077;

    #[test]
    fn test() {
        solve_print_and_check(solve_0077, 71);
    }
}
