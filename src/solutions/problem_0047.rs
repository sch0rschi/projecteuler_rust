pub fn solve_0047() -> u64 {
    const LIMIT: usize = 150_000;
    let mut factor_count = vec![0u8; LIMIT + 1];
    let mut consecutive = 0usize;

    for i in 2..=LIMIT {
        if factor_count[i] == 0 {
            for j in (i..=LIMIT).step_by(i) {
                factor_count[j] += 1;
            }
        }

        if factor_count[i] == 4 {
            consecutive += 1;
            if consecutive == 4 {
                return (i - 3) as u64;
            }
        } else {
            consecutive = 0;
        }
    }

    panic!("A solution should have been found.");
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
