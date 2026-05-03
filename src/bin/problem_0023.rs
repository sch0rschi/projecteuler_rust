use projecteuler::evaluation_helper::solve_print_and_check;

const LIMIT: usize = 28123;

fn main() {
    solve_print_and_check(solve_0023, 4179871);
}

fn solve_0023() -> usize {
    let divisor_sums = compute_divisor_sums();

    let mut is_abundant = [false; LIMIT + 1];
    let mut abundants = Vec::with_capacity(7000);

    for i in 2..=LIMIT {
        if divisor_sums[i] > i {
            is_abundant[i] = true;
            abundants.push(i);
        }
    }

    (1..LIMIT).filter(|n| !can_be_written_as_sum(&is_abundant, &abundants, *n)).sum()
}

fn can_be_written_as_sum(is_abundant: &[bool; 28124], abundants: &[usize], n: usize) -> bool {
    for &a in abundants {
        if a > n / 2 {
            return false;
        }
        if is_abundant[n - a] {
            return true;
        }
    }
    false
}

fn compute_divisor_sums() -> Vec<usize> {
    let mut sums = vec![0; LIMIT + 1];

    for i in 1..=LIMIT / 2 {
        for j in (2 * i..=LIMIT).step_by(i) {
            sums[j] += i;
        }
    }

    sums
}
