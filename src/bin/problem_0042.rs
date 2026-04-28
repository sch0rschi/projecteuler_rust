use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::word_score::score;
use std::fs;

fn main() {
    solve_print_and_check(solve_0042, 162);
}

fn solve_0042() -> usize {
    let content = fs::read_to_string("resources/0042_words.txt").expect("Failed to read file");

    content[1..content.len() - 1]
        .split("\",\"")
        .filter(|word| is_triangle_number(score(word)))
        .count()

}

fn is_triangle_number(x: i32) -> bool {
    if x <= 0 {
        return false;
    }
    let discriminant = 1 + 8 * x as u64;
    let sqrt = integer_sqrt(discriminant);
    sqrt * sqrt == discriminant && (sqrt - 1).is_multiple_of(2)
}

fn integer_sqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut s = (n as f64).sqrt() as u64;
    while s * s > n {
        s -= 1;
    }
    while (s + 1) * (s + 1) <= n {
        s += 1;
    }
    s
}
