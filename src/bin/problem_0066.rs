use projecteuler::continued_fractions::get_continued_fraction_sequence;
use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::pells_equation::get_min_x;

fn main() {
    solve_print_and_check(solve_0066, 661);
}

// help by: https://en.wikipedia.org/wiki/Pell%27s_equation
fn solve_0066() -> u64 {
    (1..=1_000)
        .map(|d| {
            let sequence = get_continued_fraction_sequence(d);
            let min_x = get_min_x(&sequence, d);
            (d, min_x)
        })
        .max_by(|(_, x1), (_, x2)| x1.cmp(x2))
        .unwrap()
        .0

}
