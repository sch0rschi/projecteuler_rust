use num_bigint::BigUint;

pub fn solve_0016() -> u64 {
    let mut n = BigUint::from(2u64);
    n = n.pow(1000);

    let mut sum = 0u64;

    for byte in n.to_radix_be(10) {
        sum += byte as u64;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0016::solve_0016;

    #[test]
    fn test() {
        solve_print_and_check(solve_0016, 1366);
    }
}
