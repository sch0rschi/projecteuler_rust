use crate::libs::digits::get_digit_count_encoding_15_max;

pub fn solve_0052() -> u64 {
    let mut from = 10000;

    loop {
        from *= 10;
        let end = from * 10 / 6;

        for i in (from + 8..=end).step_by(9) {
            let base = get_digit_count_encoding_15_max(i);

            if get_digit_count_encoding_15_max(2 * i) != base {
                continue;
            }

            if get_digit_count_encoding_15_max(3 * i) != base {
                continue;
            }

            if get_digit_count_encoding_15_max(4 * i) != base {
                continue;
            }

            if get_digit_count_encoding_15_max(5 * i) != base {
                continue;
            }

            if get_digit_count_encoding_15_max(6 * i) != base {
                continue;
            }

            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0052::solve_0052;

    #[test]
    fn test() {
        solve_print_and_check(solve_0052, 142857);
    }
}
