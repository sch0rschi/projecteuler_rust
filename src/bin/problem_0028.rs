use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0028();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(669171001, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0028() -> i32 {
    let mut sum = 1;
    let mut last = 1;
    for i in 1..=500 {
        let new_corner = last + 2 * i;
        last = new_corner + 6 * i;
        sum += 2 * new_corner;
        sum += 2 * last;
    }
    sum
}