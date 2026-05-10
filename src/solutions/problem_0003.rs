pub fn solve_0003() -> u64 {
    largest_prime_factor(600_851_475_143)
}

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut factor = 3;

    while factor * factor <= n {
        if n.is_multiple_of(factor) {
            n /= factor;
        } else {
            factor += 2;
        }
    }

    n
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0003::solve_0003;

    #[test]
    fn test() {
        solve_print_and_check(solve_0003, 6857);
    }
}
