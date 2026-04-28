use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::primes::Primes;

fn main() {
    solve_print_and_check(solve_0027, -59231);
}

fn solve_0027() -> i64 {
    let primes = Primes::primes_inclusive(1000);
    let primes_list = &primes.primes_list;

    let mut max_n = 0;
    let mut max_product = 0;
    for &b in primes_list.iter().take_while(|&p| *p <= 1000) {
        let b = b as i64;
        if b <= max_n {
            break;
        }
        for a in -999..=999 {
            let mut n: i64 = 0;
            loop {
                let polynomial_value = n * n + a * n + b;
                if polynomial_value >= 0 && primes.is_prime(polynomial_value as u64) {
                    n += 1;
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
