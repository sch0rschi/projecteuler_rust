use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::word_score::score;
use std::fs;

fn main() {
    solve_print_and_check(solve_0022, 871198282);
}

fn solve_0022() -> u32 {
    let content = fs::read("resources/0022_names.txt").unwrap();

    let mut buckets: Vec<Vec<&[u8]>> = (0..26 * 26)
        .map(|_| Vec::with_capacity(16))
        .collect::<Vec<_>>();

    content
        .split(|&b| b == b'"')
        .filter(|&name| !name.is_empty() && name[0] != b',')
        .for_each(|name| {
            let idx = ((name[0] - b'A') as usize) * 26
                + ((name.get(1).copied().unwrap_or(b'A') - b'A') as usize);
            buckets[idx].push(name);
        });

    buckets.iter_mut().for_each(|b| b.sort_unstable());

    buckets
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(i, name)| (i as u32 + 1) * score(name))
        .sum()
}
