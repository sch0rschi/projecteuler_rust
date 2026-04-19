use std::time::Instant;
use projecteuler::primes::Primes;

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

        let primes = Primes::primes_inclusive(upper);
        let primes_list = &primes.primes_list;

        for consecutive_prime_pair in primes_list.windows(2) {
            let [lower_prime, upper_prime] = consecutive_prime_pair else {
                todo!()
            };
            for composite in ((*lower_prime + 2)..*upper_prime).step_by(2) {
                let check = check_composite(composite, &primes);
                if !check {
                    return composite;
                }
            }
        }
    }
}

fn check_composite(composite: u64, primes: &Primes) -> bool {
    let mut n = 1;
    while 2 * n * n < composite {
        let remainder = composite - 2 * n * n;
        if primes.is_prime(remainder) {
            return true;
        }
        n += 1;
    }
    false
}
