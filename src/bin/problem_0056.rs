use num_bigint::BigUint;
use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0056, 972);
}

fn solve_0056() -> u64 {
    let mut max_digit_sum = 0;

    for i in 1u16..100 {
        let a = BigUint::from(i);
        let mut power = BigUint::from(1u8);

        for _ in 1..100 {
            power *= &a;

            let digit_sum = digit_sum(&power);
            max_digit_sum = max_digit_sum.max(digit_sum);
        }
    }

    max_digit_sum
}

fn digit_sum(n: &BigUint) -> u64 {
    n.to_string().bytes().map(|b| (b - b'0') as u64).sum()
}
