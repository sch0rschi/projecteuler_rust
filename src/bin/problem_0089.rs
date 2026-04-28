use numerus::int_to_roman_upper;
use numerus::roman_to_int;
use projecteuler::evaluation_helper::solve_print_and_check;
use std::fs;

fn main() {
    solve_print_and_check(solve_0089, 743);
}

fn solve_0089() -> usize {
    fs::read_to_string("resources/0089_roman.txt")
        .expect("failed to read input file")
        .lines()
        .map(parse_and_diff)
        .sum::<usize>()

}

fn parse_and_diff(roman: &str) -> usize {
    let value = roman_to_int(roman).unwrap();
    let canonical = int_to_roman_upper(value).unwrap();
    roman.len() - canonical.len()
}
