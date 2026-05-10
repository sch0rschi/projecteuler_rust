use num_integer::gcd;

const LIMIT: u64 = 50;


pub fn solve_0091() -> u64 {
    let trivial = 3 * LIMIT * LIMIT;
    let mut non_trivial = 0;
    for y in 1..=LIMIT {
        for x in y..=LIMIT {
            let gcd = gcd(x, y);

            // 90 degrees counterclockwise
            let remaining_y = LIMIT - y;
            let remaining_x = x;
            let y_step = x / gcd;
            let x_step = y / gcd;
            let min = (remaining_y / y_step).min(remaining_x / x_step);
            non_trivial += min;

            // 90 degrees clockwise
            if x > y {
                let remaining_y = y;
                let remaining_x = LIMIT - x;
                let min = (remaining_y / y_step).min(remaining_x / x_step);
                non_trivial += min;
            }
        }
    }
    trivial + 2 * non_trivial
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0091::solve_0091;

    #[test]
    fn test() {
        solve_print_and_check(solve_0091, 14234);
    }
}
