use projecteuler::primes::{primes_inclusive, unique_prime_factors, Primes};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0047();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(134043, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0047() -> u64 {
    let mut upper = 1024;

    upper *= 2;
    let Primes {
        prime_sieve: _,
        prime_list: list,
    } = primes_inclusive(upper);

    let mut c_1 = 644;
    let mut c_1_factors = unique_prime_factors(c_1, &list);
    let mut c_2 = 645;
    let mut c_2_factors = unique_prime_factors(c_2, &list);
    let mut c_3 = 646;
    let mut c_3_factors = unique_prime_factors(c_3, &list);
    let mut c_4 = 647;
    let mut c_4_factors = unique_prime_factors(c_4, &list);

    loop {
        if c_1_factors.len() == 4
            && c_2_factors.len() == 4
            && c_3_factors.len() == 4
            && c_4_factors.len() == 4
        {
            return c_1;
        }

        c_1 = c_2;
        c_1_factors = c_2_factors;
        c_2 = c_3;
        c_2_factors = c_3_factors;
        c_3 = c_4;
        c_3_factors = c_4_factors;
        c_4 += 1;
        c_4_factors = unique_prime_factors(c_4, &list);
    }
}
