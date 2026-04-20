use num_integer::gcd;
use std::time::Instant;

static LIMIT: u64 = 50;
fn main() {
    let start = Instant::now();
    let result = solve_0091();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(14234, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0091() -> u64 {
    let trivial = 3 * LIMIT * LIMIT;
    let mut non_trivial = 0;
    for y in 1..=LIMIT {
        for x in y..=LIMIT {
            let gcd = gcd(x, y);

            // 90 degrees counterclockwise
            let remaining_y = LIMIT - y;
            let remaining_x = x;
            let y_step = x / gcd;
            let x_step = y / gcd;
            let min = (remaining_y / y_step).min(remaining_x / x_step);
            non_trivial += min;

            // 90 degrees clockwise
            if x > y {
                let remaining_y = y;
                let remaining_x = LIMIT - x;
                let min = (remaining_y / y_step).min(remaining_x / x_step);
                non_trivial += min;
            }
        }
    }
    trivial + 2 * non_trivial
}
