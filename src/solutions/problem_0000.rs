const LIMIT: u64 = 504000;

pub fn solve_0000() -> u64 {
    let m: u64 = LIMIT / 2;
    m * (2 * m - 1) * (2 * m + 1) / 3
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0000::solve_0000;

    #[test]
    fn test() {
        solve_print_and_check(solve_0000, 21337343999916000);
    }
}
