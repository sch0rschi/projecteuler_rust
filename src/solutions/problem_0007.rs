const LIMIT: usize = 10001;

pub fn solve_0007() -> usize {
    primal::StreamingSieve::nth_prime(LIMIT)
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0007::solve_0007;

    #[test]
    fn test() {
        solve_print_and_check(solve_0007, 104743);
    }
}
