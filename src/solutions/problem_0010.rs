use crate::libs::primes::Primes;

const LIMIT: u64 = 2_000_000;

pub fn solve_0010() -> u64 {
    let primes = Primes::primes_inclusive(LIMIT);
    primes.primes_list.iter().sum::<u64>()
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
