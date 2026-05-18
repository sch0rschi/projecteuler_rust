pub fn solve_0064() -> usize {
    (1..=10_000u64).filter(|&n| has_odd_period(n)).count()
}

fn has_odd_period(n: u64) -> bool {
    let a0 = (n as f64).sqrt() as u64;
    if a0 * a0 == n {
        return false;
    }

    let mut m = 0u64;
    let mut d = 1u64;
    let mut a = a0;
    let mut period = 0usize;

    loop {
        m = d * a - m;
        d = (n - m * m) / d;
        a = (a0 + m) / d;
        period += 1;

        if a == 2 * a0 {
            return period % 2 == 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0064::solve_0064;

    #[test]
    fn test() {
        solve_print_and_check(solve_0064, 1322);
    }
}
