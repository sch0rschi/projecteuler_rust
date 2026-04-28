use projecteuler::evaluation_helper::solve_print_and_check;
use std::iter::successors;

fn main() {
    solve_print_and_check(solve_0063, 49);
}

fn solve_0063() -> usize {
    (1..=9u128)
        .flat_map(|base| {
            successors(Some((base, 1u128)), move |&(p, low)| {
                Some((p * base, low * 10))
            })
            .take_while(|&(power, low)| power >= low && power < 10 * low)
        })
        .count()

}
