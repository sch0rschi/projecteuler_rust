use crate::libs::primes::Primes;

const LIMIT: usize = 1000;
const PRIMES_ESTIMATE: usize = 80;

pub fn solve_0027() -> i64 {
    let primes = Primes::primes_inclusive(PRIMES_ESTIMATE * PRIMES_ESTIMATE + PRIMES_ESTIMATE * LIMIT + LIMIT);

    let mut max_n = 0;
    let mut max_product = 0;

    for b in primes.single_iterator().take_while(|&p| p < LIMIT + 1) {
        let b = b as i64;
        let (start, step) = if b == 2 {
            (-998i64, 2usize)
        } else {
            (-999i64, 2usize)
        };

        for a in (start..=999i64).step_by(step) {
            let mut value = b + a + 1;
            let mut delta = a + 3;

            for n in 1.. {
                if value >= 0 && primes.is_prime(value as usize) {
                    value += delta;
                    delta += 2;
                } else {
                    if n > max_n {
                        max_n = n;
                        max_product = a * b;
                    }
                    break;
                }
            }
        }
    }

    max_product
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0027::solve_0027;

    #[test]
    fn test() {
        solve_print_and_check(solve_0027, -59231);
    }
}
