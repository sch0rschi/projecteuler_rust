use crate::libs::primes::Primes;

const LIMIT: usize = 2_000_000;

pub fn solve_0010() -> usize {
    let primes = Primes::primes_inclusive(LIMIT);
    primes.single_iterator().sum()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0010::solve_0010;

    #[test]
    fn test() {
        solve_print_and_check(solve_0010, 142913828922);
    }
}
