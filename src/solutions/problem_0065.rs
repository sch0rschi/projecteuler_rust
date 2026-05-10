use std::mem::swap;

use num_bigint::BigInt;


pub fn solve_0065() -> u64 {
    let mut numerator = BigInt::from(1u64);
    let mut denominator = BigInt::from(0u64);

    for i in (1..=100).rev() {
        let a = match i % 3 {
            2 => 2 * (i / 3 + 1),
            _ => 1,
        };

        numerator += a * &denominator;
        swap(&mut numerator, &mut denominator);
    }

    numerator += BigInt::from(2u64) * &denominator;
    swap(&mut numerator, &mut denominator);

    denominator
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0065::solve_0065;

    #[test]
    fn test() {
        solve_print_and_check(solve_0065, 272);
    }
}
