use crate::libs::permutations::prev_permutation;
use primal::is_prime;

pub fn solve_0041() -> u64 {
    let mut digits = [7u64, 6, 5, 4, 3, 2, 1];
    let mut done = false;

    std::iter::from_fn(move || {
        if done {
            return None;
        }
        let n = digits.iter().fold(0u64, |acc, &d| acc * 10 + d);
        if !prev_permutation(&mut digits) {
            done = true;
        }
        Some(n)
    })
    .find(|&n| is_prime(n))
    .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0041::solve_0041;

    #[test]
    fn test() {
        solve_print_and_check(solve_0041, 7652413);
    }
}
