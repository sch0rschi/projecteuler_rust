use crate::libs::digits::get_digit_occurrence_mask;

const MULTIPLES: &[u64] = &[17, 13, 11, 7, 5, 3, 2];
const INDICES: &[u64] = &[
    999 / 17,
    999 / 13,
    999 / 11,
    999 / 7,
    999 / 5,
    999 / 3,
    999 / 2,
];
const POW_10: &[u64] = &[1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];

pub fn solve_0043() -> u64 {
    let mut sum = 0;
    recursion(0, 0, &mut sum);
    sum
}

fn recursion(stage_idx: usize, number: u64, sum: &mut u64) {
    let multiple = MULTIPLES[stage_idx];
    let shift_10 = POW_10[stage_idx];
    let distinct = stage_idx + 3;

    for i in 0..=INDICES[stage_idx] {
        let stage_number = i * multiple;

        if stage_idx > 0 && number / shift_10 != stage_number % 100 {
            continue;
        }

        let new_number = number
            + if stage_idx == 0 {
                stage_number
            } else {
                (stage_number / 100) * 100 * shift_10
            };

        let digit_mask = get_digit_occurrence_mask(new_number, distinct);
        if digit_mask.count_ones() as usize != distinct {
            continue;
        }

        if stage_idx + 1 < MULTIPLES.len() {
            recursion(stage_idx + 1, new_number, sum);
        } else {
            let missing = digit_mask.trailing_ones() as u64;
            *sum += new_number + missing * 1_000_000_000;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0043::solve_0043;

    #[test]
    fn test() {
        solve_print_and_check(solve_0043, 16695334890);
    }
}
