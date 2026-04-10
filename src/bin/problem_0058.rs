use projecteuler::primes::{primes_inclusive, Primes};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0058();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(26241, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0058() -> i32 {
    let Primes {
        prime_sieve: _,
        prime_list
    } = primes_inclusive(100_000);

    let mut result = 0;
    let mut diagonal_prime_count = 0;
    let mut diagonal_elements_count = 1;
    let mut top_right = 1;
    let mut top_left = 1;
    let mut bottom_left = 1;
    let mut adding = 0;
    for i in (3..).step_by(2) {
        // we go from 0 instead of 3
        top_right += 2 + adding;
        top_left += 4 + adding;
        bottom_left += 6 + adding;
        adding += 8;
        diagonal_elements_count += 4;
        diagonal_prime_count +=
            check_prime(top_left, prime_list.as_slice()) as u64
                + check_prime(top_right, prime_list.as_slice()) as u64
                + check_prime(bottom_left, prime_list.as_slice()) as u64;

        if 10 * diagonal_prime_count < diagonal_elements_count {
            result = i;
            break;
        }
    }

    result
}

fn check_prime(n: u64, prime_list: &[u64]) -> bool {
    let sqrt = (n as f64).sqrt() as u64 + 1;
    for &prime in prime_list {
        if prime > sqrt {
            return true;
        } else if n.is_multiple_of(prime) {
            return false;
        }
    }
    unreachable!()
}
