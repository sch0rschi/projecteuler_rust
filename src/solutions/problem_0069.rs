use crate::libs::coprimes::phi;
use crate::libs::primes::Primes;


pub fn solve_0069() -> i32 {
    let primes = Primes::primes_inclusive(1_000);

    (2..=1_000_000)
        .map(|n| {
            let factors = primes.unique_prime_factors(n as u64);
            let phi = phi(n as u64, &factors);
            (n, n as u64 * 1_000_000u64 / phi)
        })
        .max_by_key(|&(_, ratio)| ratio)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0069::solve_0069;

    #[test]
    fn test() {
        solve_print_and_check(solve_0069, 510510);
    }
}
