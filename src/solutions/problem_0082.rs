use itertools::Itertools;

const INPUT: &str = include_str!("../../resources/0082_matrix.txt");

pub fn solve_0082() -> u32 {
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

    let mut dp: Vec<u32> = (0..rows).map(|r| weights[r][0]).collect();

    #[allow(clippy::needless_range_loop)]
    for column in 1..cols {
        let mut next: Vec<u32> = (0..rows).map(|r| dp[r] + weights[r][column]).collect();

        for row in 1..rows {
            next[row] = next[row].min(next[row - 1] + weights[row][column]);
        }

        for row in (0..rows - 1).rev() {
            next[row] = next[row].min(next[row + 1] + weights[row][column]);
        }

        dp = next;
    }

    *dp.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0082::solve_0082;

    #[test]
    fn test() {
        solve_print_and_check(solve_0082, 260324);
    }
}
