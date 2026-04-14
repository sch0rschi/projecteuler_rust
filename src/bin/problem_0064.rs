use std::time::Instant;
use projecteuler::continued_fractions::get_continued_fraction_sequence;

fn main() {
    let start = Instant::now();
    let result = solve_0064();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(1322, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0064() -> usize {
    (1..=10_000)
        .map(get_continued_fraction_sequence)
        .map(|sequence| sequence.len() - 1)
        .filter(|p| p % 2 == 1)
        .count()
}
