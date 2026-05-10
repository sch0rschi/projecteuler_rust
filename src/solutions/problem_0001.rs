pub fn solve_0001() -> u32 {
    let n = 999;

    let sum = |k: u32| {
        let m = n / k;
        k * m * (m + 1) / 2
    };

    sum(3) + sum(5) - sum(15)
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0001::solve_0001;

    #[test]
    fn test() {
        solve_print_and_check(solve_0001, 233168);
    }
}
