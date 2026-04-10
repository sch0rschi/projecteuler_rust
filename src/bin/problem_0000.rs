use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0000();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(21337343999916000, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0000() -> u64 {
    let count: u64 = 504000;
    let mut sum: u64 = 0;

    for n in (1..=count).step_by(2) {
        sum += n * n;
    }

    sum
}
