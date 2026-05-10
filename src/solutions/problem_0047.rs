use crate::libs::primes::Primes;


pub fn solve_0047() -> u64 {
    let mut upper = 1024;

    upper *= 2;
    let primes = Primes::primes_inclusive(upper);

    let mut c_1 = 644;
    let mut c_1_factors = primes.unique_prime_factors(c_1);
    let mut c_2 = 645;
    let mut c_2_factors = primes.unique_prime_factors(c_2);
    let mut c_3 = 646;
    let mut c_3_factors = primes.unique_prime_factors(c_3);
    let mut c_4 = 647;
    let mut c_4_factors = primes.unique_prime_factors(c_4);

    loop {
        if c_1_factors.len() == 4
            && c_2_factors.len() == 4
            && c_3_factors.len() == 4
            && c_4_factors.len() == 4
        {
            return c_1;
        }

        c_1 = c_2;
        c_1_factors = c_2_factors;
        c_2 = c_3;
        c_2_factors = c_3_factors;
        c_3 = c_4;
        c_3_factors = c_4_factors;
        c_4 += 1;
        c_4_factors = primes.unique_prime_factors(c_4);
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0047::solve_0047;

    #[test]
    fn test() {
        solve_print_and_check(solve_0047, 134043);
    }
}
