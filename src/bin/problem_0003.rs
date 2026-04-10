use std::cmp::max;
use std::time::Instant;

enum FactorResult {
    Factors(i64, i64),
    Prime(i64),
}

fn main() {
    let start = Instant::now();
    let result = solve_0003();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(6857, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0003() -> i64 {
    let number: i64 = 600851475143;
    factorization_recursion(&number)
}

fn factorization_recursion(n: &i64) -> i64 {
    let result = divisors_or_prime(n);
    match result {
        FactorResult::Prime(p) => p,
        FactorResult::Factors(n_1, n_2) => {
            let factor_1 = factorization_recursion(&n_1);
            if factor_1 >= n_2 {
                return factor_1;
            }
            let factor_2 = factorization_recursion(&n_2);
            max(factor_1, factor_2)
        }
    }
}

fn divisors_or_prime(n: &i64) -> FactorResult {
    let square_root_n = n.isqrt();

    for potential_factor in (2..=square_root_n).rev() {
        if n % potential_factor == 0 {
            return FactorResult::Factors(potential_factor, n / potential_factor);
        }
    }
    FactorResult::Prime(*n)
}
