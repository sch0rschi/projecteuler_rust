use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0006();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(25164150, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0006() -> i32 {
    let mut squares_sum = 0;
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
        squares_sum += i * i;
    }
    sum * sum - squares_sum
}
