use std::iter::successors;


pub fn solve_0063() -> usize {
    (1..=9u128)
        .flat_map(|base| {
            successors(Some((base, 1u128)), move |&(p, low)| {
                Some((p * base, low * 10))
            })
                .take_while(|&(power, low)| power >= low && power < 10 * low)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0063::solve_0063;

    #[test]
    fn test() {
        solve_print_and_check(solve_0063, 49);
    }
}
