use std::time::Instant;

const LIMIT: usize = 55374+1;
fn main() {
    let start = Instant::now();
    let result = solve_0078();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(55374, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

// using the partition function from https://en.wikipedia.org/wiki/Partition_function_(number_theory)
fn solve_0078() -> u64 {
    let mut dp = vec![0i64; LIMIT + 1];
    dp[0] = 1;

    let modulo = 1_000_000;

    for n in 1..=LIMIT {
        let mut k = 1;
        let mut val = 0;

        loop {
            let g1 = k * (3 * k - 1) / 2;
            let g2 = k * (3 * k + 1) / 2;

            if g1 > n {
                break;
            }

            let sign = if k % 2 == 0 { -1 } else { 1 };

            val += sign * dp[n - g1];

            if g2 <= n {
                val += sign * dp[n - g2];
            }

            k += 1;
        }

        dp[n] = val % modulo;
        if dp[n] < 0 {
            dp[n] += modulo;
        }

        if dp[n] == 0 {
            return n as u64;
        }
    }

    panic!()
}