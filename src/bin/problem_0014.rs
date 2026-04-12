use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0014();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(837799, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0014() -> u64 {
    let mut cache = vec![0u16; 2_000_000];
    cache[1] = 1;

    let mut max_len = 0;
    let mut max_len_value = 1;

    for i in 500_000..1_000_000 {
        let len = collatz_len(i, &mut cache);
        if len > max_len {
            max_len = len;
            max_len_value = i;
        }
    }

    max_len_value
}

fn collatz_len(mut n: u64, cache: &mut [u16]) -> u16 {
    let mut steps = 0;
    let start = n;

    while n > 1 && (n as usize >= cache.len() || cache[n as usize] == 0) {
        if n.is_multiple_of(2) {
            n /= 2;
            steps += 1;
        } else {
            n = (3 * n + 1).div_ceil(2);
            steps += 2;
        }
    }

    let known = if n < cache.len() as u64 {
        cache[n as usize]
    } else {
        0
    };

    let total = steps + known;

    if start < cache.len() as u64 {
        cache[start as usize] = total;
    }

    total
}