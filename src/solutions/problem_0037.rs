use crate::libs::primes_u32::is_prime;
use smallvec::SmallVec;

const FIRST_DIGITS: [u32; 4] = [2, 3, 5, 7];


pub fn solve_0037() -> u32 {
    let mut sum = 0u32;
    let mut count = 0u32;
    let mut current: SmallVec<[u32; 32]> = SmallVec::from_slice(&FIRST_DIGITS);
    let mut next: SmallVec<[u32; 32]> = SmallVec::with_capacity(32);

    while count < 11 {
        next.clear();
        for &n in &current {
            let pow10 = 10u32.pow(n.ilog10() + 1);
            for d in [3, 7] {
                let candidate = n * 10 + d;
                if is_prime(candidate) {
                    next.push(candidate);
                    if is_left_truncatable(candidate, pow10) {
                        sum += candidate;
                        count += 1;
                    }
                }
            }
            for d in [1, 9] {
                let candidate = n * 10 + d;
                if is_prime(candidate) {
                    next.push(candidate);
                }
            }
        }
        std::mem::swap(&mut current, &mut next);
    }

    sum
}

fn is_left_truncatable(n: u32, mut pow10: u32) -> bool {
    while pow10 > 1 {
        let div = n / pow10;
        let truncated = n - div * pow10;
        if !is_prime(truncated) {
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
