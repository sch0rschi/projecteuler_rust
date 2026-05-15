use smallvec::SmallVec;

const FIRST_DIGITS: [usize; 4] = [2, 3, 5, 7];

pub fn solve_0037() -> usize {
    let mut sum = 0usize;
    let mut count = 0usize;
    let mut current: SmallVec<[usize; 32]> = SmallVec::from_slice(&FIRST_DIGITS);
    let mut next: SmallVec<[usize; 32]> = SmallVec::with_capacity(32);

    while count < 11 {
        next.clear();
        for &n in &current {
            let pow10 = 10usize.pow(n.ilog10() + 1);
            for d in [3, 7] {
                let candidate = n * 10 + d;
                if primal::is_prime(candidate as u64) {
                    next.push(candidate);
                    if is_left_truncatable(candidate, pow10) {
                        sum += candidate;
                        count += 1;
                    }
                }
            }
            for d in [1, 9] {
                let candidate = n * 10 + d;
                if primal::is_prime(candidate as u64) {
                    next.push(candidate);
                }
            }
        }
        std::mem::swap(&mut current, &mut next);
    }

    sum
}

fn is_left_truncatable(n: usize, mut pow10: usize) -> bool {
    while pow10 > 1 {
        let div = n / pow10;
        let truncated = n - div * pow10;
        if !primal::is_prime(truncated as u64) {
            return false;
        }
        pow10 /= 10;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0037::solve_0037;

    #[test]
    fn test() {
        solve_print_and_check(solve_0037, 748317);
    }
}
