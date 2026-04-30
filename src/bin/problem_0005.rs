use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::primes::Primes;

fn main() {
    solve_print_and_check(solve_0005, 232792560);
}

const LIMIT: u64 = 20;

fn solve_0005() -> u64 {
    let mut result = 1u64;
    let primes = Primes::primes_inclusive(LIMIT);

    for p in primes.primes_list {
        let exp = (LIMIT as f64).log(p as f64).floor() as u32;
        result *= p.pow(exp);
    }

    result
}
