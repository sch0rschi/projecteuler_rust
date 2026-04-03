use std::fs;
use std::time::Instant;
use projecteuler::word_score::score;

fn main() {
    let start = Instant::now();

    let content = fs::read_to_string("resources/0042_words.txt").expect("Failed to read file");

    let count = content[1..content.len() - 1]
        .split("\",\"")
        .filter(|word| is_triangle_number(score(word)))
        .count();

    println!("{}", count);
    println!("Elapsed: {:?}", start.elapsed());
}

fn is_triangle_number(x: i32) -> bool {
    if x <= 0 { return false; }
    let discriminant = 1 + 8 * x as u64;
    let sqrt = integer_sqrt(discriminant);
    sqrt * sqrt == discriminant && (sqrt - 1).is_multiple_of(2)
}

fn integer_sqrt(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut s = (n as f64).sqrt() as u64;
    while s * s > n { s -= 1; }
    while (s + 1) * (s + 1) <= n { s += 1; }
    s
}
