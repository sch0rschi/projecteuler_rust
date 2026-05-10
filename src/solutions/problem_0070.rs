use crate::libs::digits::get_digit_count_encoding_15_max;
use crate::libs::totients::get_totient_sieve;
use num_integer::Roots;

pub fn solve_0070() -> u64 {
    let limit = 10_000_000usize;
    let totient_sieve = get_totient_sieve(limit);

    let mut best_n = u64::MAX.sqrt();
    let mut best_ph = 1u64;

    for (n, &phi) in totient_sieve.iter().enumerate().skip(2) {
        let ph = phi as u64;
        let n64 = n as u64;

        if ph == 0 || ph == n64 {
            continue;
        }

        if n64 * best_ph < best_n * ph && is_permutation(n64, ph) {
            best_n = n64;
            best_ph = ph;
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
