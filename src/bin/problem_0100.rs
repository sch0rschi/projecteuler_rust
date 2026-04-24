use std::time::Instant;
static LIMIT: u64 = 1_000_000_000_000;

fn main() {
    let start = Instant::now();
    let result = solve_0100();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(756872327473, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0100() -> u64 {
    // Transforming 2 * blue_disks * (blue_disks - 1) == limit * (limit - 1)
    // into bell equation, then setup recurrence.

    let mut x_precious2: u64 = 1;
    let mut x_previous: u64 = 3;
    let mut y_previous2: u64 = 1;
    let mut y_previous: u64 = 4;

    loop {
        let x_next = 6 * x_previous - x_precious2 - 2;
        let y_next = 6 * y_previous - y_previous2 - 2;
        x_precious2 = x_previous;
        x_previous = x_next;
        y_previous2 = y_previous;
        y_previous = y_next;

        if y_previous > LIMIT {
            return x_previous;
        }
    }
}
