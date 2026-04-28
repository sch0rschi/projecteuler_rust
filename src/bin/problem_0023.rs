use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0023, 4179871);
}

fn solve_0023() -> usize {
    let limit = 28123;

    let divisor_sums = compute_divisor_sums(limit);

    let abundant: Vec<usize> = (2..=limit).filter(|&i| divisor_sums[i] > i).collect();

    let mut can_be_written = vec![false; limit + 1];

    for (i, &n1) in abundant.iter().enumerate() {
        for &n2 in &abundant[i..] {
            let sum = n1 + n2;
            if sum > limit {
                break;
            }
            can_be_written[sum] = true;
        }
    }

    (1..=limit)
        .filter(|&i| !can_be_written[i])
        .sum()

}

fn compute_divisor_sums(limit: usize) -> Vec<usize> {
    let mut sums = vec![0; limit + 1];

    for i in 1..=limit / 2 {
        for j in (2 * i..=limit).step_by(i) {
            sums[j] += i;
        }
    }

    sums
}
