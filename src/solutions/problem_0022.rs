use crate::libs::word_score::score;

const CONTENT: &str = include_str!("../../resources/0022_names.txt");

pub fn solve_0022() -> u32 {
    let mut buckets: Vec<Vec<&str>> = (0..26 * 26).map(|_| Vec::with_capacity(16)).collect();

    CONTENT
        .split('"')
        .filter(|name| !name.is_empty() && !name.starts_with(','))
        .for_each(|name| {
            let bytes = name.as_bytes();

            let idx = ((bytes[0] - b'A') as usize) * 26
                + ((bytes.get(1).copied().unwrap_or(b'A') - b'A') as usize);

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

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0022::solve_0022;

    #[test]
    fn test() {
        solve_print_and_check(solve_0022, 871198282);
    }
}
