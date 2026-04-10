use projecteuler::divisors::proper_divisor_sum;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0021();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(31626, result);
    assert!(duration < std::time::Duration::from_secs(1));
}
fn solve_0021() -> u64 {
    let mut d: [u64; 10_000] = [0; 10_000];
    let mut amicable_numbers_sum = 0;
    for i in 1..10_000 {
        let proper_divisor_sum = proper_divisor_sum(i);
        d[i as usize] = proper_divisor_sum;
        if proper_divisor_sum < i && d[proper_divisor_sum as usize] == i {
            amicable_numbers_sum += i;
            amicable_numbers_sum += proper_divisor_sum;
        }
    }
    amicable_numbers_sum
}
