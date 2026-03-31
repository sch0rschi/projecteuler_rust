use projecteuler::primes::primes_inclusive;

fn main() {
    let (_, primes) = primes_inclusive(2_000_000);
    println!("{}", primes.iter().sum::<i64>());
}