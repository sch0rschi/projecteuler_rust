use num_integer::Integer;


pub fn solve_0073() -> i32 {
    let limit = 12_000;
    let lower_fraction_nominator = 1;
    let lower_fraction_denominator = 3;
    let upper_fraction_nominator = 1;
    let upper_fraction_denominator = 2;
    let mut count = 0;
    for nominator in 1..limit {
        for denominator in 2..=limit {
            if nominator * lower_fraction_denominator > denominator * lower_fraction_nominator
                && nominator * upper_fraction_denominator < denominator * upper_fraction_nominator
                && nominator.gcd(&denominator) == 1
            {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0073::solve_0073;

    #[test]
    fn test() {
        solve_print_and_check(solve_0073, 7295372);
    }
}
