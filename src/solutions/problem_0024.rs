use num_integer::Integer;

const FAC_9: usize = 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2;
const N_TH_PERMUTATION: usize = 1_000_000;

pub fn solve_0024() -> usize {
    let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut result = 0;
    let mut div: usize;
    let mut remaining = N_TH_PERMUTATION - 1;
    let mut number_of_open_digits = digits.len();
    let mut permutation_range = FAC_9;

    for _ in 0..9 {
        (div, remaining) = remaining.div_rem(&permutation_range);
        result = result * 10 + digits[div];
        digits.copy_within(div + 1..number_of_open_digits, div);
        number_of_open_digits -= 1;
        permutation_range /= number_of_open_digits;
    }
    result * 10 + digits[0]
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0024::solve_0024;

    #[test]
    fn test() {
        solve_print_and_check(solve_0024, 2783915460);
    }
}
