const THRESHOLD: u64 = 1_000_000;
const CAP: u64 = THRESHOLD + 1;

pub fn solve_0053() -> u32 {

    let mut count = 0u32;

    for n in 1u64..=100 {
        let mut c = 1u64;
        for k in 1..=n {
            c = (c * (n - k + 1) / k).min(CAP);
            if c > THRESHOLD {
                count += (n + 1 - 2 * k) as u32;
                break;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0053::solve_0053;

    #[test]
    fn test() {
        solve_print_and_check(solve_0053, 4075);
    }
}
