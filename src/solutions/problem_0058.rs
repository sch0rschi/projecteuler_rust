pub fn solve_0058() -> u32 {
    let mut result = 0;
    let mut diagonal_prime_count = 0;
    let mut diagonal_elements_count = 1;
    let mut top_right = 1;
    let mut top_left = 1;
    let mut bottom_left = 1;
    let mut adding = 0;
    for i in (3..).step_by(2) {
        // we go from 0 instead of 3
        top_right += 2 + adding;
        top_left += 4 + adding;
        bottom_left += 6 + adding;
        adding += 8;
        diagonal_elements_count += 4;
        diagonal_prime_count += primal::is_prime(top_left) as usize
            + primal::is_prime(top_right) as usize
            + primal::is_prime(bottom_left) as usize;

        if 10 * diagonal_prime_count < diagonal_elements_count {
            result = i;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0058::solve_0058;

    #[test]
    fn test() {
        solve_print_and_check(solve_0058, 26241);
    }
}
