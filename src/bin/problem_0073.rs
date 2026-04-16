use num_integer::Integer;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0073();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(7295372, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0073() -> u64 {
    let limit = 12_000;
    let lower_fraction_nominator = 1;
    let lower_fraction_denominator = 3;
    let upper_fraction_nominator = 1;
    let upper_fraction_denominator = 2;
    let mut count = 0;
    for nominator in 1..limit {
        for denominator in 2..=limit {
            if nominator * lower_fraction_denominator > denominator * lower_fraction_nominator
                && nominator * upper_fraction_denominator < denominator * upper_fraction_nominator
                && nominator.gcd(&denominator) == 1
            {
                count += 1;
            }
        }
    }
    count
}
