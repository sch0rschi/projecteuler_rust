use projecteuler::primes;
fn main() {
    let primes = primes::find_first_n_primes(10001);

    println!("{}", primes.last().unwrap());
}
