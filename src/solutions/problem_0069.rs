pub fn solve_0069() -> usize {
    [2usize, 3, 5, 7, 11, 13, 17, 19]
        .iter()
        .scan(1usize, |acc, &prime| {
            (*acc * prime <= 1_000_000).then(|| {
                *acc *= prime;
                *acc
            })
        })
        .last()
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
