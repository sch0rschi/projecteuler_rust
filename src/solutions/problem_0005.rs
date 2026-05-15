use crate::libs::primes::Primes;

const LIMIT: usize = 20;

pub fn solve_0005() -> usize {
    let mut result = 1;
    let primes = Primes::primes_inclusive(LIMIT);

    for p in primes.single_iterator() {
        let exp = (LIMIT as f64).log(p as f64).floor() as usize;
        result *= p.pow(exp as u32);
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
