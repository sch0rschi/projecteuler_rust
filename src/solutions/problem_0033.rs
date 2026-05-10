use num_integer::Integer;


pub fn solve_0033() -> u16 {
    let mut result_numerator = 1u16;
    let mut result_denominator = 1u16;

    for numerator_tens in 1u16..10 {
        for numerator_ones in 1u16..10 {
            let numerator = 10 * numerator_tens + numerator_ones;
            for denominator_ones in (numerator_tens + 1)..10 {
                let denominator_tens = numerator_ones;
                let denominator = 10 * denominator_tens + denominator_ones;

                if numerator * denominator_ones == denominator * numerator_tens {
                    result_numerator *= numerator;
                    result_denominator *= denominator;
                    let gcd = result_numerator.gcd(&result_denominator);
                    result_numerator /= gcd;
                    result_denominator /= gcd;
                }
            }
        }
    }

    result_denominator
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0033::solve_0033;

    #[test]
    fn test() {
        solve_print_and_check(solve_0033, 100);
    }
}
