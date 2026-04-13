use std::iter::successors;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0063();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(49, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0063() -> u8 {
    (1..=9u128)
        .flat_map(|base| {
            successors(Some((base, 1u128)), move |&(p, low)| {
                Some((p * base, low * 10))
            })
            .take_while(|&(power, low)| power >= low && power < 10 * low)
        })
        .count() as u8
}
