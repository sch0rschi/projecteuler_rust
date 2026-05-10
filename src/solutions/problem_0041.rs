use crate::libs::pandigital::is_1_to_length_pandigital;
use crate::libs::primes::Primes;


pub fn solve_0041() -> u64 {
    let primes = Primes::primes_inclusive(7654321);
    let primes_list = &primes.primes_list;

    *primes_list
        .iter()
        .rev()
        .find(|&&p| is_1_to_length_pandigital(p))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0041::solve_0041;

    #[test]
    fn test() {
        solve_print_and_check(solve_0041, 7652413);
    }
}
