use projecteuler::primes::{primes_inclusive, Primes};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0046();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(5777, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0046() -> u64 {
    let mut upper = 1024;

    loop {
        upper *= 2;

        let Primes { prime_sieve: sieve, prime_list: list } = primes_inclusive(upper);

        for consecutive_prime_pair in list.windows(2) {
            let [lower_prime, upper_prime] = consecutive_prime_pair else {
                todo!()
            };
            for composite in ((*lower_prime + 2)..*upper_prime).step_by(2) {
                let check = check_composite(composite, &sieve);
                if !check {
                    return composite;
                }
            }
        }
    }
}

fn check_composite(composite: u64, sieve: &[bool]) -> bool {
    let mut n = 1;
    while 2 * n * n < composite {
        let remainder = composite - 2 * n * n;
        if sieve[remainder as usize] {
            return true;
        }
        n += 1;
    }
    false
}
