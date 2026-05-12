use crate::libs::primes::Primes;

pub fn solve_0049() -> String {
    let primes = Primes::primes_inclusive(9999);

    let mut candidates: Vec<(u32, u32)> = primes
        .primes_list
        .iter()
        .filter(|&&p| p >= 1000)
        .map(|&p| (digit_fingerprint(p as u32), p as u32))
        .collect();

    candidates.sort_unstable();

    let mut i = 0;
    while i < candidates.len() {
        let fp = candidates[i].0;
        let j = candidates[i..].partition_point(|&(f, _)| f == fp) + i;
        let group = &candidates[i..j];

        if group.len() >= 3 {
            for a in 0..group.len() {
                let pa = group[a].1;
                for b in (a + 1)..group.len() {
                    let pb = group[b].1;
                    let pc = 2 * pb - pa;
                    if pc > 9999 {
                        break;
                    }
                    if group[b..].binary_search_by_key(&pc, |&(_, p)| p).is_ok() && pa != 1487 {
                        return format!("{}{}{}", pa, pb, pc);
                    }
                }
            }
        }

        i = j;
    }

    panic!("A solution should have been found.");
}

#[inline(always)]
fn digit_fingerprint(n: u32) -> u32 {
    let mut digits = [n / 1000, (n / 100) % 10, (n / 10) % 10, n % 10];
    digits.sort_unstable();
    digits[0] | digits[1] << 4 | digits[2] << 8 | digits[3] << 12
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0049::solve_0049;

    #[test]
    fn test() {
        solve_print_and_check(solve_0049, "296962999629".to_string());
    }
}
