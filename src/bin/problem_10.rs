use projecteuler::primes::find_primes_up_to_exclusive;

fn main() {
    let primes = find_primes_up_to_exclusive(2000000);
    println!("{}", primes.iter().sum::<i64>());
}