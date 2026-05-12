use crate::libs::totients::get_totient_sieve;

pub fn solve_0069() -> usize {
    let phi = get_totient_sieve(1_000_000);

    (2..=1_000_000)
        .max_by_key(|&n| n as u64 * 1_000_000u64 / phi[n] as u64)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0069::solve_0069;

    #[test]
    fn test() {
        solve_print_and_check(solve_0069, 510510);
    }
}
