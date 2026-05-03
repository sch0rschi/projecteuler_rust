use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::exponentiation::mod_pow;
use projecteuler::primes::Primes;

const LIMIT: usize = 1000;

fn main() {
    solve_print_and_check(solve_0026, 983);
}

// https://en.wikipedia.org/wiki/Repeating_decimal#Fractions_with_prime_denominators
// https://en.wikipedia.org/wiki/Multiplicative_order
fn solve_0026() -> u64 {
    let primes = Primes::primes_inclusive(LIMIT as u64);

    let mut best_cycle_length = 0;
    let mut best_cycle_length_prime = None;

    for &prime in primes.primes_list.iter().rev() {
        if prime < best_cycle_length {
            break;
        }
        if [2, 5].contains(&prime) {
            continue;
        }

        let cycle_length = multiplicative_order_10(prime, &primes);

        if cycle_length > best_cycle_length {
            best_cycle_length = cycle_length;
            best_cycle_length_prime = Some(prime);
        }
    }

    best_cycle_length_prime.expect("There should be a prime with positive cycle length.")
}

fn multiplicative_order_10(p: u64, primes: &Primes) -> u64 {
    let phi = p - 1;

    let mut order = phi;

    let factors = primes.unique_prime_factors(phi);

    for factor in factors {
        while order % factor == 0 && mod_pow(10, order / factor, p) == 1 {
            order /= factor;
        }
    }

    order
}
