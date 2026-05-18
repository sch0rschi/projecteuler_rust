use num_bigint::BigUint;

pub fn solve_0080() -> u32 {
    let ten_pow_198 = BigUint::from(10u32).pow(198);

    (2u32..100)
        .filter(|&n| {
            let r = (n as f64).sqrt() as u32;
            r * r != n
        })
        .map(|n| get_root_decimal_digit_sum(n, &ten_pow_198))
        .sum()
}

fn get_root_decimal_digit_sum(n: u32, ten_pow_198: &BigUint) -> u32 {
    (BigUint::from(n) * ten_pow_198)
        .sqrt()
        .to_radix_be(10)
        .iter()
        .take(100)
        .map(|&d| d as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0080::solve_0080;

    #[test]
    fn test() {
        solve_print_and_check(solve_0080, 40886);
    }
}
