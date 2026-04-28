use std::time::Instant;
use bitvec::bitvec;

const LIMIT: usize = 10_000_000;
fn main() {
    let start = Instant::now();
    let result = solve_0092();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(8581146, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0092() -> u64 {
    let mut count = 0;
    let mut seen_1 = bitvec![0; 568]; // 7 * 81 + 1
    let mut seen_89 = bitvec![0; 568];
    let mut chain: Vec<usize> = Vec::new();
    for mut i in 1..=LIMIT {
        chain.clear();
        i = next(i);
        chain.push(i);
        loop {
            if i == 1 || seen_1[i] {
                for &in_1 in &chain {
                    seen_1.set(in_1, true);
                }
                break;
            }
            if i == 89 || seen_89[i] {
                for &in_89 in &chain {
                    seen_89.set(in_89, true);
                }
                count += 1;
                break;
            }
            chain.push(i);
            i = next(i);
        }

    }
    count
}

fn next(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        sum+= (n % 10) * (n % 10);
        n /= 10;
    }
    sum
}
