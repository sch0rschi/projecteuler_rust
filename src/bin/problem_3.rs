use std::cmp::max;

enum FactorResult {
    Factors(i64, i64),
    Prime(i64),
}

fn main() {
    let number: i64 = 600851475143;
    let largest_prime_factor = factorization_recursion(&number);
    println!("{}", largest_prime_factor);
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
