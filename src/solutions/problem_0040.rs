const POW10: [i32; 7] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];

pub fn solve_0040() -> i32 {
    let mut d_index = 1;
    let mut fraction_length = 0;
    let mut start_number = 1;
    let mut number_length = 1;
    let mut number_count = 9;

    let mut product = 1;

    while fraction_length <= 1_000_000 {
        let mut taking = number_length * number_count;

        while fraction_length + taking >= d_index {
            let offset = (d_index - fraction_length - 1) / number_length;
            let number = start_number + offset;

            start_number = number + 1;
            d_index *= 10;

            taking -= (offset + 1) * number_length;
            fraction_length += (offset + 1) * number_length;

            let in_number_index = (d_index - fraction_length - 1) % number_length;

            let digit = (number / POW10[(number_length - in_number_index - 1) as usize]) % 10;

            product *= digit;
        }

        fraction_length += taking;

        number_length += 1;
        number_count *= 10;
        start_number = POW10[(number_length - 1) as usize];
    }

    product
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0040::solve_0040;

    #[test]
    fn test() {
        solve_print_and_check(solve_0040, 210);
    }
}
