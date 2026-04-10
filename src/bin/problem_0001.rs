use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0001();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(233168, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

pub fn solve_0001() -> i32 {
    let mut sum = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    sum
}
