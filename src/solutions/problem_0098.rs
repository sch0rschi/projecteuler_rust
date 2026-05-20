use ahash::AHashMap;

const INPUT: &str = include_str!("../../resources/0098_words.txt");

pub fn solve_0098() -> u64 {
    let mut groups: AHashMap<[u8; 26], Vec<&'static str>> = AHashMap::default();

    for word in INPUT.split(',') {
        let w = word.trim_matches('"');
        groups.entry(signature(w)).or_default().push(w);
    }

    // keep only anagram groups, grouped by length
    let mut words_by_len: AHashMap<usize, Vec<Vec<&'static str>>> = AHashMap::default();

    for g in groups.into_values() {
        if g.len() > 1 {
            let len = g[0].len();
            words_by_len.entry(len).or_default().push(g);
        }
    }

    let max_len = words_by_len.keys().copied().max().unwrap();
    let max_square = 10u64.pow(max_len as u32) - 1;
    let max_root = (max_square as f64).sqrt() as u64;

    let mut map = [-1i8; 26];

    for n in (1..=max_root).rev() {
        let sq = n * n;
        let len = digits(sq);

        let Some(groups) = words_by_len.get(&len) else {
            continue;
        };

        for group in groups {
            for &w in group {
                map.fill(-1);

                if !set_character_to_digit_mapping(sq, w, &mut map) {
                    continue;
                }

                for &other in group {
                    if other == w {
                        continue;
                    }

                    if let Some(candidate) = apply_mapping(other, &map)
                        && is_square(candidate)
                    {
                        return sq;
                    }
                }
            }
        }
    }

    unreachable!()
}

#[inline]
fn signature(s: &str) -> [u8; 26] {
    let mut freq = [0u8; 26];
    for &b in s.as_bytes() {
        freq[(b - b'A') as usize] += 1;
    }
    freq
}

#[inline]
fn is_square(n: u64) -> bool {
    let r = (n as f64).sqrt() as u64;
    r * r == n || (r + 1) * (r + 1) == n
}

#[inline]
fn digits(mut n: u64) -> usize {
    let mut len = 0;
    while n > 0 {
        n /= 10;
        len += 1;
    }
    len
}

#[inline]
fn set_character_to_digit_mapping(mut sq: u64, w: &str, map: &mut [i8; 26]) -> bool {
    let bytes = w.as_bytes();
    let mut i = bytes.len();
    let mut used = [false; 10];

    while sq > 0 {
        if i == 0 {
            return false;
        }

        i -= 1;
        let d = (sq % 10) as i8;
        sq /= 10;

        let idx = (bytes[i] - b'A') as usize;

        match map[idx] {
            -1 => {
                if used[d as usize] {
                    return false;
                }
                map[idx] = d;
                used[d as usize] = true;
            }
            x if x != d => return false,
            _ => {}
        }
    }

    i == 0 && sq == 0
}

#[inline]
fn apply_mapping(word: &str, mapping: &[i8; 26]) -> Option<u64> {
    let bytes = word.as_bytes();

    if mapping[(bytes[0] - b'A') as usize] == 0 {
        return None;
    }

    let mut n = 0;
    for &b in bytes {
        let d = mapping[(b - b'A') as usize];
        if d == -1 {
            return None;
        }
        n = n * 10 + d as u64;
    }

    Some(n)
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0098::solve_0098;

    #[test]
    fn test() {
        solve_print_and_check(solve_0098, 18769);
    }
}
