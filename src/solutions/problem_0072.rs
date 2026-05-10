use crate::libs::coprimes::phi;
use crate::libs::primes::Primes;
use num_integer::Roots;


pub fn solve_0072() -> u64 {
    let limit = 1_000_000;
    let primes = Primes::primes_inclusive(limit.sqrt());

    (2..=limit)
        .map(|n| {
            let factors = primes.unique_prime_factors(n);
            phi(n, &factors)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0072::solve_0072;

    #[test]
    fn test() {
        solve_print_and_check(solve_0072, 303963552391);
    }
}
