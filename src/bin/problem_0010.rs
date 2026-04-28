use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::primes::Primes;

const LIMIT: u64 = 2_000_000;

fn main() {
    solve_print_and_check(solve_0010, 142913828922);
}

fn solve_0010() -> u64 {
    let primes = Primes::primes_inclusive(LIMIT);
    primes.primes_list.iter().sum::<u64>()
}
