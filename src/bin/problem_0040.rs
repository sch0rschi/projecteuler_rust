use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0040, 210);
}

fn solve_0040() -> i32 {
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
            let mut digit = number / 10_i32.pow(number_length as u32 - in_number_index as u32 - 1);
            digit %= 10;
            product *= digit;
        }

        fraction_length += taking;

        number_length += 1;
        number_count *= 10;

        start_number = 10_i32.pow((number_length - 1) as u32);
    }

    product
}
