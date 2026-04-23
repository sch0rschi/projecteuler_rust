use projecteuler::divisors::ProperDivisorSums;
use std::time::Instant;

const LIMIT: usize = 10_000;

fn main() {
    let start = Instant::now();
    let result = solve_0021();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(31626, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0021() -> u32 {
    let divisor_sums = ProperDivisorSums::new(LIMIT);
    let mut d: [u32; LIMIT] = [0; LIMIT];
    let mut amicable_numbers_sum = 0;
    for i in 1u32..LIMIT as u32 {
        let proper_divisor_sum = divisor_sums.get(i);
        d[i as usize] = proper_divisor_sum;
        if proper_divisor_sum < i && d[proper_divisor_sum as usize] == i {
            amicable_numbers_sum += i;
            amicable_numbers_sum += proper_divisor_sum;
        }
    }
    amicable_numbers_sum
}
