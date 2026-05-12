use crate::libs::totients::get_totient_sieve;

pub fn solve_0072() -> u64 {
    let phi = get_totient_sieve(1_000_000);
    phi[2..=1_000_000].iter().map(|&p| p as u64).sum()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0072::solve_0072;

    #[test]
    fn test() {
        solve_print_and_check(solve_0072, 303963552391);
    }
}
