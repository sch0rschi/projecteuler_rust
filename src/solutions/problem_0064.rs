use crate::libs::continued_fractions::get_continued_fraction_sequence;


pub fn solve_0064() -> usize {
    (1..=10_000)
        .map(get_continued_fraction_sequence)
        .map(|sequence| sequence.len() - 1)
        .filter(|p| p % 2 == 1)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0064::solve_0064;

    #[test]
    fn test() {
        solve_print_and_check(solve_0064, 1322);
    }
}
