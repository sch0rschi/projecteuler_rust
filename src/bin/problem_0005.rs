use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0005();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(232792560, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0005() -> u64 {
    2 * 2 * 2 * 2 * 3 * 3 * 5 * 7 * 11 * 13 * 17 * 19
}