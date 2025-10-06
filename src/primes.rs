pub fn find_first_n_primes(n: i64) -> Vec<i64> {
    let mut primes: Vec<i64> = [2, 3].to_vec();

    for _ in 3..=n {
        let next_prime = find_next_prime(&primes);
        primes.push(next_prime);
    }
    primes
}

pub fn find_primes_up_to_exclusive(limit: i64) -> Vec<i64> {
    let mut primes: Vec<i64> = [2, 3].to_vec();

    loop {
        let next_prime = find_next_prime(&primes);
        if next_prime >= limit {
            break;
        }
        primes.push(next_prime);
    }
    primes
}

fn find_next_prime(previous_primes: &[i64]) -> i64 {
    let last_prime = *previous_primes.last().unwrap();
    for candidate_prime in ((last_prime + 2)..).step_by(2) {
        let candidate_prime_sqrt = candidate_prime.isqrt();

        if previous_primes
            .iter()
            .take_while(|&&p| p <= candidate_prime_sqrt)
            .all(|&previous_prime| candidate_prime % previous_prime != 0)
        {
            return candidate_prime;
        }
    }
    unreachable!("The loop should always return a prime");
}
