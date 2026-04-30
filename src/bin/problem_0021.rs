use projecteuler::evaluation_helper::solve_print_and_check;

const LIMIT: usize = 10_000;

fn main() {
    solve_print_and_check(solve_0021, 31626);
}

fn solve_0021() -> usize {
    let mut divisor_sums = [0usize; LIMIT];

    for i in 1..LIMIT {
        for j in (2 * i..LIMIT).step_by(i) {
            divisor_sums[j] += i;
        }
    }

    divisor_sums
        .iter()
        .enumerate()
        .filter(|&(i, &divisor_sum)| {
            divisor_sum < LIMIT && divisor_sums[divisor_sum] == i && divisor_sum != i
        })
        .map(|(i, _)| i)
        .sum()
}
