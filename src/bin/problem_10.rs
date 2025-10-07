use projecteuler::primes::find_primes_up_to_inclusive;

fn main() {
    let primes = find_primes_up_to_inclusive(2_000_000);
    println!("{}", primes.iter().sum::<i64>());
}