use crate::libs::primes::Primes;


pub fn solve_0046() -> u64 {
    let mut upper = 1024;

    loop {
        upper *= 2;

        let primes = Primes::primes_inclusive(upper);
        let primes_list = &primes.primes_list;

        for window in primes_list.windows(2) {
            for composite in ((window[0] + 2)..window[1]).step_by(2) {
                let check = check_composite(composite, &primes);
                if !check {
                    return composite;
                }
            }
        }
    }
}

fn check_composite(composite: u64, primes: &Primes) -> bool {
    let mut n = 1;
    while 2 * n * n < composite {
        let remainder = composite - 2 * n * n;
        if primes.is_prime(remainder) {
            return true;
        }
        n += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0046::solve_0046;

    #[test]
    fn test() {
        solve_print_and_check(solve_0046, 5777);
    }
}
