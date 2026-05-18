const INPUT: &str = include_str!("../../resources/0089_roman.txt");

pub fn solve_0089() -> usize {
    INPUT
        .lines()
        .map(|line| line.len() - canonical_len(parse(line)))
        .sum()
}

fn parse(s: &str) -> u32 {
    let digit = |c| match c {
        b'I' => 1,
        b'V' => 5,
        b'X' => 10,
        b'L' => 50,
        b'C' => 100,
        b'D' => 500,
        b'M' => 1000,
        _ => 0,
    };
    let b = s.as_bytes();
    let mut n = 0u32;
    for i in 0..b.len() {
        let cur = digit(b[i]);
        let nxt = if i + 1 < b.len() { digit(b[i + 1]) } else { 0 };
        if cur < nxt {
            n -= cur;
        } else {
            n += cur;
        }
    }
    n
}

fn canonical_len(mut n: u32) -> usize {
    const NUMERALS: &[(u32, usize)] = &[
        (1000, 1),
        (900, 2),
        (500, 1),
        (400, 2),
        (100, 1),
        (90, 2),
        (50, 1),
        (40, 2),
        (10, 1),
        (9, 2),
        (5, 1),
        (4, 2),
        (1, 1),
    ];
    let mut len = 0;
    for &(val, clen) in NUMERALS {
        len += (n / val) as usize * clen;
        n %= val;
    }
    len
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0089::solve_0089;

    #[test]
    fn test() {
        solve_print_and_check(solve_0089, 743);
    }
}
