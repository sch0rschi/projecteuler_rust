use num_integer::Integer;

pub struct Primes {
    pub prime_sieve: Vec<bool>,
    pub prime_list:  Vec<i64>,
}

pub fn find_first_n_primes(n: i64) -> Vec<i64> {
    let mut primes: Vec<i64> = [2, 3].to_vec();

    for _ in 3..=n {
        let next_prime = find_next_prime(&primes);
        primes.push(next_prime);
    }
    primes
}

pub fn primes_inclusive(limit: i64) -> Primes {
    let sieve = prime_sieve_up_to_inclusive(limit);
    let list = sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i as i64) } else { None })
        .collect();
    Primes { prime_sieve: sieve.clone(), prime_list: list }
}

pub fn prime_sieve_up_to_inclusive(limit: i64) -> Vec<bool> {
    if limit < 2 {
        return vec![false; (limit + 1) as usize];
    }

    let mut is_prime = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrt_limit = (limit as f64).sqrt() as i64;

    for num in 2..=sqrt_limit {
        if is_prime[num as usize] {
            let mut multiple = num * num;
            while multiple <= limit {
                is_prime[multiple as usize] = false;
                multiple += num;
            }
        }
    }

    is_prime
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

pub fn add_next_prime(previous_primes: &mut Vec<i64>) {
    let next_prime = find_next_prime(previous_primes);
    previous_primes.push(next_prime);
}

pub fn unique_prime_factors(n: i64, list: &[i64]) -> Vec<i64> {
    let mut factors: Vec<i64> = Vec::new();

    let mut number = n;
    for &prime in list.iter() {
        if number.is_multiple_of(&prime) {
            number /= prime;
            factors.push(prime);
            while number.is_multiple_of(&prime) {
                number /= prime;
            }
        }

         if number == 1 {
            break;
        }
    }

    factors
}