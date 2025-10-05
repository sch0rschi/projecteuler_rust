fn main() {
    let mut primes: Vec<i32> = [2, 3].to_vec();

    for _ in 3..=10001 {
        let next_prime = find_next_prime(&mut primes);
        println!("{}", next_prime);
        primes.push(next_prime);
    }

    println!("{}", primes.last().unwrap());
}

fn find_next_prime(previous_primes: &mut Vec<i32>) -> i32 {
    let last_prime = *previous_primes.last().unwrap();
    for candidate_prime in ((last_prime + 2)..).step_by(2) {
        let candidate_prime_sqrt = candidate_prime.isqrt();

        if previous_primes
            .iter()
            .take_while(|&&p| p <= candidate_prime_sqrt)
            .all(|&previous_prime| candidate_prime % previous_prime != 0)
        {
            previous_primes.push(candidate_prime);
            return candidate_prime;
        }
    }
    unreachable!("The loop should always return a prime");
}
