use projecteuler::continued_fractions::get_continued_fraction_sequence;
use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0064, 1322);
}

fn solve_0064() -> usize {
    (1..=10_000)
        .map(get_continued_fraction_sequence)
        .map(|sequence| sequence.len() - 1)
        .filter(|p| p % 2 == 1)
        .count()

}
