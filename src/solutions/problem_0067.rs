use itertools::Itertools;
use std::fs;


pub fn solve_0067() -> u64 {
    let mut triangle = fs::read_to_string("resources/0067_triangle.txt")
        .expect("Failed to read file")
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect_vec();

    for row in (0..triangle.len() - 1).rev() {
        for i in 0..triangle[row].len() {
            triangle[row][i] += triangle[row + 1][i].max(triangle[row + 1][i + 1]);
        }
    }

    triangle[0][0]
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0067::solve_0067;

    #[test]
    fn test() {
        solve_print_and_check(solve_0067, 7273);
    }
}
