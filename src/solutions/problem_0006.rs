const LIMIT: u32 = 100u32;

pub fn solve_0006() -> u32 {
    let sum = LIMIT * (LIMIT + 1) / 2;
    let sum_sq = LIMIT * (LIMIT + 1) * (2 * LIMIT + 1) / 6;
    sum * sum - sum_sq
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0006::solve_0006;

    #[test]
    fn test() {
        solve_print_and_check(solve_0006, 25164150);
    }
}
