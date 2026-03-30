use projecteuler::pandigital::is_1_to_length_pandigital;
use projecteuler::primes::find_primes_up_to_inclusive;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let primes = find_primes_up_to_inclusive(7654321);

    let max_pandigital_prime = primes
        .iter()
        .rev()
        .find(|&&p| is_1_to_length_pandigital(p))
        .unwrap();

    println!("{}", max_pandigital_prime);
    println!("{:?}", start.elapsed());
}
