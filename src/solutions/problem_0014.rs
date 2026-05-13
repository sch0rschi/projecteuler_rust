const LIMIT: usize = 1_000_000;

// https://risingentropy.com/2019/06/12/record-breaking-collatz-chains/
pub fn solve_0014() -> usize {
    let mut chain_lengths = vec![0u32; LIMIT];
    chain_lengths[2] = 1;
    chain_lengths[3] = 7;
    chain_lengths[4] = 2;
    chain_lengths[5] = 5;
    chain_lengths[6] = 8;
    chain_lengths[7] = 16;
    chain_lengths[8] = 3;
    chain_lengths[9] = 19;
    chain_lengths[10] = 6;
    chain_lengths[11] = 14;

    for base in (12..LIMIT - 12).step_by(12) {
        // n mod 12 == 0
        chain_lengths[base] = chain_lengths[base / 2] + 1;
        // n mod 12 == 2
        let chain_length_2 = chain_lengths[(base + 2) / 2] + 1;
        chain_lengths[base + 2] = chain_length_2;
        // n mod 12 == 4
        chain_lengths[base + 4] = chain_lengths[(base + 4) / 2] + 1;
        // n mod 12 == 6
        chain_lengths[base + 6] = chain_lengths[(base + 6) / 2] + 1;
        // n mod 12 == 8
        let chain_length_8 = chain_lengths[(base + 8) / 2] + 1;
        chain_lengths[base + 8] = chain_length_8;
        // n mod 12 == 10
        chain_lengths[base + 10] = chain_lengths[(base + 10) / 2] + 1;

        // n mod 12 == 1
        let n = base + 1;
        let target = (3 * n + 1) / 4;
        chain_lengths[n] = chain_lengths[target] + 3;
        // n mod 12 == 5
        let n = base + 5;
        let target = (3 * n + 1) / 4;
        chain_lengths[n] = chain_lengths[target] + 3;

        // n mod 12 == 3
        chain_lengths[base + 3] = chain_length_2;
        // n mod 12 == 9
        chain_lengths[base + 9] = chain_length_8;

        // n mod 12 == 7
        let n = base + 7;
        let (next, chain_length) = build_chain(n);
        chain_lengths[n] = chain_length + chain_lengths[next];
        // n mod 12 == 11
        let n = base + 11;
        let (next, chain_length) = build_chain(n);
        chain_lengths[n] = chain_length + chain_lengths[next];
    }
    let base = 12 * (LIMIT / 12);
    chain_lengths[base] = chain_lengths[base / 2] + 1;
    let n = base + 1;
    let target = (3 * n + 1) / 4;
    chain_lengths[n] = chain_lengths[target] + 3;
    chain_lengths[base + 2] = chain_lengths[(base + 2) / 2] + 1;
    chain_lengths[base + 3] = chain_lengths[base + 2];

    chain_lengths
        .iter()
        .enumerate()
        .max_by_key(|(_, l)| *l)
        .unwrap()
        .0
}

#[inline(always)]
fn build_chain(n: usize) -> (usize, u32) {
    let mut next = n;
    let mut count = 0;
    loop {
        next = next_collatz(next);
        count += 1;
        if next < n {
            return (next, count);
        }
    }
}

#[inline(always)]
fn next_collatz(n: usize) -> usize {
    if n.is_multiple_of(2) { n / 2 } else { 3 * n + 1 }
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0014::solve_0014;

    #[test]
    fn test() {
        solve_print_and_check(solve_0014, 837799);
    }
}
