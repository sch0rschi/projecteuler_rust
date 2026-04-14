use std::time::Instant;
use projecteuler::continued_fractions::{get_continued_fraction_sequence};
use projecteuler::pells_equation::get_min_x;

fn main() {
    let start = Instant::now();
    let result = solve_0066();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(661, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

// help by: https://en.wikipedia.org/wiki/Pell%27s_equation
fn solve_0066() -> u64 {
    (1..=1_000)
        .map(|d| {
            let sequence = get_continued_fraction_sequence(d);
            let min_x = get_min_x(&sequence, d);
            (d, min_x)
        })
        .max_by(|(_, x1), (_, x2)| x1.cmp(x2))
        .unwrap()
        .0
}
