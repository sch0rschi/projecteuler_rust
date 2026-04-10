use projecteuler::word_score::score;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0022();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(871198282, result);
    assert!(duration < std::time::Duration::from_secs(1));
}
fn solve_0022() -> i32 {
    let content = fs::read_to_string("resources/0022_names.txt").expect("Failed to read file");

    let mut names: Vec<&str> = content[1..content.len() - 1].split("\",\"").collect();

    names.sort_unstable();

    let score_sum: i32 = names
        .iter()
        .enumerate()
        .map(|(i, &name)| (i as i32 + 1) * score(name))
        .sum();

    score_sum
}
