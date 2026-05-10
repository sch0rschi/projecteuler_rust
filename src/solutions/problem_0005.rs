use crate::libs::primes::Primes;

const LIMIT: u64 = 20;

pub fn solve_0005() -> u64 {
    let mut result = 1u64;
    let primes = Primes::primes_inclusive(LIMIT);

    for p in primes.primes_list {
        let exp = (LIMIT as f64).log(p as f64).floor() as u32;
        result *= p.pow(exp);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0005::solve_0005;

    #[test]
    fn test() {
        solve_print_and_check(solve_0005, 232792560);
    }
}
