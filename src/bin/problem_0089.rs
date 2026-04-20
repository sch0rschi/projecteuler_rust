use numerus::int_to_roman_upper;
use numerus::roman_to_int;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0089();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(743, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0089() -> usize {
    fs::read_to_string("resources/0089_roman.txt")
        .expect("failed to read input file")
        .lines()
        .map(parse_and_diff)
        .sum()
}

fn parse_and_diff(roman: &str) -> usize {
    let value = roman_to_int(roman).unwrap();
    let canonical = int_to_roman_upper(value).unwrap();
    roman.len() - canonical.len()
}
