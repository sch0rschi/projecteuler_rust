use num_integer::Integer;


pub fn solve_0071() -> i32 {
    let limit = 1_000_000;
    let mut closest_proper_numerator = 2;
    let mut closest_proper_denominator = 5;
    let mut numerator = 2;
    let mut denominator = 5;

    loop {
        if denominator > limit {
            break;
        }
        if numerator.gcd(&denominator) == 1
            && numerator * closest_proper_denominator > closest_proper_numerator * denominator
        {
            closest_proper_numerator = numerator;
            closest_proper_denominator = denominator;
        }
        if (numerator + 1) * 7 < 3 * denominator {
            numerator += 1;
        } else {
            denominator += 1;
        }
    }

    closest_proper_numerator
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0071::solve_0071;

    #[test]
    fn test() {
        solve_print_and_check(solve_0071, 428570);
    }
}
