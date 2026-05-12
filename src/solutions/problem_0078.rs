pub fn solve_0078() -> usize {
    const MODULO: i32 = 1_000_000;
    const LIMIT: usize = 60_000;

    let mut dp = vec![0i32; LIMIT + 1];
    dp[0] = 1;

    let pents: Vec<(usize, usize)> = (1..)
        .map(|k: usize| (k * (3 * k - 1) / 2, k * (3 * k + 1) / 2))
        .take_while(|&(g1, _)| g1 <= LIMIT)
        .collect();

    for n in 1..=LIMIT {
        let mut val = 0i32;

        for (i, &(g1, g2)) in pents.iter().enumerate() {
            if g1 > n { break; }
            let sign = if i % 2 == 0 { 1 } else { -1 };
            val += sign * dp[n - g1];
            if g2 <= n {
                val += sign * dp[n - g2];
            }
        }

        dp[n] = val.rem_euclid(MODULO);

        if dp[n] == 0 {
            return n;
        }
    }

    panic!("A solution should have been found.");
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0078::solve_0078;

    #[test]
    fn test() {
        solve_print_and_check(solve_0078, 55374);
    }
}
