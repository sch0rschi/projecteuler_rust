pub fn solve_0047() -> u64 {
    const LIMIT: usize = 150_000;
    let mut factor_count = vec![0u8; LIMIT + 1];

    for i in 2..=LIMIT {
        if factor_count[i] == 0 {
            for j in (i..=LIMIT).step_by(i) {
                factor_count[j] += 1;
            }
        }
    }

    factor_count
        .windows(4)
        .position(|w| w.iter().all(|&c| c == 4))
        .unwrap() as u64
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0047::solve_0047;

    #[test]
    fn test() {
        solve_print_and_check(solve_0047, 134043);
    }
}
