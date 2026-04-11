use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0034();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(40730, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

#[inline(always)]
fn factorial_table() -> [u64; 10] {
    let mut f = [1u64; 10];
    for i in 2..10 {
        f[i] = f[i - 1] * i as u64;
    }
    f
}

fn solve_0034() -> u64 {
    let fact = factorial_table();

    // upper bound: 7 * 9! = 2,540,160
    let limit = 2_540_160;

    let mut sum = 0u64;

    for i in 10..limit {
        let mut n = i;
        let mut s = 0u64;

        while n > 0 {
            s += fact[(n % 10) as usize];
            n /= 10;
        }

        if s == i as u64 {
            sum += i as u64;
        }
    }

    sum
}