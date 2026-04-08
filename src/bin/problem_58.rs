use projecteuler::primes::{primes_inclusive, Primes};
use std::time::Instant;
fn main() {
    let start = Instant::now();

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

    println!("{}", result);
    println!("Elapsed: {:?}", start.elapsed());
}

fn check_prime(n: i64, prime_list: &[i64]) -> bool {
    let sqrt = (n as f64).sqrt() as i64 + 1;
    for &prime in prime_list {
        if prime > sqrt {
            return true;
        } else if n % prime == 0 {
            return false;
        }
    }
    assert!(false, "unreachable");
    unreachable!()
}
