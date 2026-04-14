use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let start = Instant::now();
    let result = solve_0067();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(7273, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0067() -> u64 {
    let mut triangle = fs::read_to_string("resources/0067_triangle.txt")
        .expect("Failed to read file")
        .lines()
        .map(|line| {line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>()})
        .collect_vec();

    for row in (0..triangle.len() - 1).rev() {
        for i in 0..triangle[row].len() {
            triangle[row][i] += triangle[row + 1][i].max(triangle[row + 1][i + 1]);
        }
    }

    triangle[0][0]

}
