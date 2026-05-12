const LIMIT_POW_10: usize = 7;
const ONE_STEP_RANGE: usize = LIMIT_POW_10 * 9 * 9;
const DIGIT_SQUARES: [usize; 10] = [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];

pub fn solve_0092() -> u32 {
    let mut leads_to_89_in_one_step = [false; ONE_STEP_RANGE+1];
    for (i, leads_to_89_in_one_step) in leads_to_89_in_one_step.iter_mut().enumerate().skip(1).take(ONE_STEP_RANGE) {
        *leads_to_89_in_one_step = reaches_89(i);
    }

    let mut new_counts = [0u32; ONE_STEP_RANGE+1];
    let mut counts = [0u32; ONE_STEP_RANGE+1];
    counts[0] = 1;

    for _ in 0..LIMIT_POW_10 {
        new_counts.fill(0);
        for (index, &count) in counts.iter().enumerate() {
            if count == 0 {
                continue;
            }
            for digit_square in DIGIT_SQUARES {
                let index_added_digit = index + digit_square;
                new_counts[index_added_digit] += count;
            }
        }
        std::mem::swap(&mut counts, &mut new_counts);
    }

    counts
        .iter()
        .enumerate()
        .filter(|(i, _)| leads_to_89_in_one_step[*i])
        .map(|(_, &count)| count)
        .sum()
}

fn reaches_89(mut i: usize) -> bool {
    loop {
        if i == 89 {
            return true;
        }
        if i == 1 {
            return false;
        }
        i = digit_square_sum(i);
    }
}

fn digit_square_sum(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        let modulo = n % 10;
        sum += DIGIT_SQUARES[modulo];
        n /= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0092::solve_0092;

    #[test]
    fn test() {
        solve_print_and_check(solve_0092, 8581146);
    }
}
