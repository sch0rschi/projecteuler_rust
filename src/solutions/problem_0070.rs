use crate::libs::digits::get_digit_count_encoding_15_max;
use crate::libs::primes::Primes;
use itertools::Itertools;
use num_integer::Roots;

const LIMIT: usize = 10_000_000;

// This constant is an "educated" guesses
const FACTOR_LOWER_LIMIT: usize = 2_000;
const FACTOR_UPPER_LIMIT: usize = LIMIT / FACTOR_LOWER_LIMIT;

pub fn solve_0070() -> usize {
    let primes = Primes::primes_inclusive(FACTOR_UPPER_LIMIT);
    let relevant_primes = primes
        .single_iterator()
        .filter(|&p| p > FACTOR_LOWER_LIMIT && p < FACTOR_UPPER_LIMIT)
        .collect_vec();

    let mut best_n = usize::MAX.sqrt();
    let mut best_phi = 1;

    for (i1, &p1) in relevant_primes.iter().enumerate() {
        for &p2 in relevant_primes.iter().skip(i1) {
            let n = p1 * p2;
            if n > LIMIT - 1 {
                break;
            }
            let phi = n - p1 - p2 + 1;
            if n * best_phi < best_n * phi && is_permutation(n as u64, phi as u64) {
                best_n = n;
                best_phi = phi;
            }
        }
    }

    best_n
}

#[inline]
fn is_permutation(a: u64, b: u64) -> bool {
    get_digit_count_encoding_15_max(a) == get_digit_count_encoding_15_max(b)
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0070::solve_0070;

    #[test]
    fn test() {
        solve_print_and_check(solve_0070, 8319823);
    }
}
