use itertools::Itertools;
use projecteuler::evaluation_helper::solve_print_and_check;
use std::fs;

fn main() {
    solve_print_and_check(solve_0067, 7273);
}

fn solve_0067() -> u64 {
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
