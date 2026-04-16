use std::time::Instant;
use num_integer::Integer;

fn main() {
    let start = Instant::now();
    let result = solve_0071();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(428570, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0071() -> u64 {
    let limit = 1_000_000;
    let mut closest_proper_numerator = 2;
    let mut closest_proper_denominator = 5;
    let mut numerator = 2;
    let mut denominator = 5;

    loop {
        if denominator > limit {
            break;
        }
        if numerator.gcd (&denominator) == 1 && numerator * closest_proper_denominator > closest_proper_numerator * denominator {
            closest_proper_numerator = numerator;
            closest_proper_denominator = denominator;
        }
        if (numerator + 1) * 7 < 3 * denominator {
            numerator += 1;
        } else {
            denominator += 1;
        }
    }

    closest_proper_numerator
}
