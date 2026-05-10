use crate::libs::primes::Primes;

const LIMIT: u64 = 10001;

pub fn solve_0007() -> u64 {
    let estimate = LIMIT * ((LIMIT as f64).ln().ceil() * 1.1) as u64;
    let primes = Primes::primes_inclusive(estimate);
    *primes.primes_list.get(LIMIT as usize - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0007::solve_0007;

    #[test]
    fn test() {
        solve_print_and_check(solve_0007, 104743);
    }
}
