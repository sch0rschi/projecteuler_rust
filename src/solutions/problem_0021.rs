const LIMIT: usize = 10_000;

pub fn solve_0021() -> usize {
    let mut divisor_sums = [0usize; LIMIT];

    for i in 1..LIMIT {
        for j in (2 * i..LIMIT).step_by(i) {
            divisor_sums[j] += i;
        }
    }

    divisor_sums
        .iter()
        .enumerate()
        .filter(|&(i, &divisor_sum)| {
            divisor_sum < LIMIT && divisor_sums[divisor_sum] == i && divisor_sum != i
        })
        .map(|(i, _)| i)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0021::solve_0021;

    #[test]
    fn test() {
        solve_print_and_check(solve_0021, 31626);
    }
}
