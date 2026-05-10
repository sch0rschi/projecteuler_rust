const N: u64 = 20;

pub fn solve_0015() -> u64 {
    let mut result: u64 = 1;

    // How many permutations of (R,R,R,R,...,D,D,D,D,...) are there?
    // Safe binomial computation
    for i in 1..=N {
        result = result * (N + i) / (i);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0015::solve_0015;

    #[test]
    fn test() {
        solve_print_and_check(solve_0015, 137846528820);
    }
}
