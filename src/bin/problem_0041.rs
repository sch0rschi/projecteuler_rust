use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::pandigital::is_1_to_length_pandigital;
use projecteuler::primes::Primes;

fn main() {
    solve_print_and_check(solve_0041, 7652413);
}

fn solve_0041() -> u64 {
    let primes = Primes::primes_inclusive(7654321);
    let primes_list = &primes.primes_list;

    *primes_list
        .iter()
        .rev()
        .find(|&&p| is_1_to_length_pandigital(p))
        .unwrap()

}
