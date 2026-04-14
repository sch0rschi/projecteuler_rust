use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0064();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(1322, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0064() -> usize {
    (1..=10_000)
        .map(find_period_length)
        .filter(|p| p % 2 == 1)
        .count()
}

fn find_period_length(n: i32) -> usize {
    let a0 = (n as f64).sqrt() as i32;
    if a0 * a0 == n {
        return 0;
    }

    let mut add = 0;
    let mut divisor = 1;
    let mut factor = a0;

    let mut period = 0;

    loop {
        add = divisor * factor - add;
        divisor = (n - add * add) / divisor;
        factor = (a0 + add) / divisor;

        period += 1;

        if factor == 2 * a0 {
            return period;
        }
    }
}
