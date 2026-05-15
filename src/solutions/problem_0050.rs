use crate::libs::primes::Primes;

pub fn solve_0050() -> usize {
    let primes = Primes::primes_inclusive(999_999);

    let mut prefix_sums = vec![0usize];
    for p in primes.single_iterator() {
        let next = prefix_sums.last().unwrap() + p;
        prefix_sums.push(next);
        if next >= 1_000_000 { break; }
    }
    let len = prefix_sums.len();

    for chain_len in (1..len).rev() {
        for start in 0..=(len - 1 - chain_len) {
            let sum = prefix_sums[start + chain_len] - prefix_sums[start];
            if sum >= 1_000_000 { break; }
            if primes.is_prime(sum) {
                return sum;
            }
        }
    }
    panic!("A solution should have been found.");
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0050::solve_0050;

    #[test]
    fn test() {
        solve_print_and_check(solve_0050, 997651);
    }
}
