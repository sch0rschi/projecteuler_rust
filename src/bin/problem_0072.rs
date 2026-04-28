use num_integer::Roots;
use projecteuler::coprimes::phi;
use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::primes::Primes;

fn main() {
    solve_print_and_check(solve_0072, 303963552391);
}

fn solve_0072() -> u64 {
    let limit = 1_000_000;
    let primes = Primes::primes_inclusive(limit.sqrt());

    (2..=limit)
        .map(|n| {
            let factors = primes.unique_prime_factors(n);
            phi(n, &factors)
        })
        .sum()

}
