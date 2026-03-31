use std::fs;
use std::time::Instant;
use projecteuler::word_score::score;

fn main() {
    let start = Instant::now();

    let content = fs::read_to_string("resources/0022_names.txt").expect("Failed to read file");

    let mut names: Vec<&str> = content[1..content.len() - 1].split("\",\"").collect();

    names.sort_unstable();

    let score_sum: i32 = names
        .iter()
        .enumerate()
        .map(|(i, &name)| (i as i32 + 1) * score(name))
        .sum();

    println!("{}", score_sum);
    println!("Elapsed: {:?}", start.elapsed());
}
