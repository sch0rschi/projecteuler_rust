use itertools::Itertools;

const INPUT: &str = include_str!("../../resources/0081_matrix.txt");

pub fn solve_0081() -> u32 {
    let weights = INPUT
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split(',')
                .map(|x| x.trim().parse::<u32>().expect("Failed to parse integer"))
                .collect_vec()
        })
        .collect_vec();

    let rows = weights.len();
    let cols = weights[0].len();
    let mut dp = vec![vec![u32::MAX; cols]; rows];

    dp[0][0] = weights[0][0];
    for c in 1..cols {
        dp[0][c] = dp[0][c - 1] + weights[0][c];
    }

    for r in 1..rows {
        dp[r][0] = dp[r - 1][0] + weights[r][0];
    }

    for r in 1..rows {
        for c in 1..cols {
            dp[r][c] = weights[r][c] + dp[r - 1][c].min(dp[r][c - 1]);
        }
    }

    dp[rows - 1][cols - 1]
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0081::solve_0081;

    #[test]
    fn test() {
        solve_print_and_check(solve_0081, 427337);
    }
}
