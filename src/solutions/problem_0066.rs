use crate::libs::continued_fractions::get_continued_fraction_sequence;
use crate::libs::pells_equation::get_min_x;


// help by: https://en.wikipedia.org/wiki/Pell%27s_equation
pub fn solve_0066() -> u64 {
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

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0066::solve_0066;

    #[test]
    fn test() {
        solve_print_and_check(solve_0066, 661);
    }
}
