use crate::libs::primes::Primes;

pub fn solve_0027() -> i64 {
    let primes = Primes::primes_inclusive(1000);
    let primes_list = &primes.primes_list;

    let mut max_n = 0;
    let mut max_product = 0;

    for &b in primes_list {
        let b = b as i64;

        for a in (-999i64..=999).step_by(2) {
            let mut value = b + a + 1;
            let mut delta = a + 3;

            for n in 1.. {
                if value >= 0 && primes.is_prime(value as u64) {
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
