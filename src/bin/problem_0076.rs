use std::time::Instant;

static LIMIT: usize = 100;
fn main() {
    let start = Instant::now();
    let result = solve_0076();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(190569291, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0076() -> u64 {
    let mut dp = vec![0u64; LIMIT + 1];
    dp[0] = 1;

    for i in 1..LIMIT {
        for j in i..=LIMIT {
            dp[j] += dp[j - i];
        }
    }

    dp[LIMIT]
}
