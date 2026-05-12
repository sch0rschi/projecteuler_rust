pub fn solve_0093() -> u32 {
    let mut max_n = 0;
    let mut max_digits = [0u32; 4];

    for a in 1..=6u32 {
        for b in a + 1..=7 {
            for c in b + 1..=8 {
                for d in c + 1..=9 {
                    let mut covered = [false; 10_000];
                    let nums = [a as f64, b as f64, c as f64, d as f64];
                    all_results(&nums, &mut covered);
                    let n = (1..).find(|&i| !covered[i]).unwrap() - 1;
                    if n > max_n {
                        max_n = n;
                        max_digits = [a, b, c, d];
                    }
                }
            }
        }
    }

    max_digits[0] * 1000 + max_digits[1] * 100 + max_digits[2] * 10 + max_digits[3]
}

fn all_results(nums: &[f64; 4], covered: &mut [bool; 10_000]) {
    let perms = [
        [0, 1, 2, 3],
        [0, 1, 3, 2],
        [0, 2, 1, 3],
        [0, 2, 3, 1],
        [0, 3, 1, 2],
        [0, 3, 2, 1],
        [1, 0, 2, 3],
        [1, 0, 3, 2],
        [1, 2, 0, 3],
        [1, 2, 3, 0],
        [1, 3, 0, 2],
        [1, 3, 2, 0],
        [2, 0, 1, 3],
        [2, 0, 3, 1],
        [2, 1, 0, 3],
        [2, 1, 3, 0],
        [2, 3, 0, 1],
        [2, 3, 1, 0],
        [3, 0, 1, 2],
        [3, 0, 2, 1],
        [3, 1, 0, 2],
        [3, 1, 2, 0],
        [3, 2, 0, 1],
        [3, 2, 1, 0],
    ];

    for p in &perms {
        let [a, b, c, d] = [nums[p[0]], nums[p[1]], nums[p[2]], nums[p[3]]];
        // all 5 distinct binary tree shapes for 4 operands:
        // shape 1: ((a ○ b) ○ c) ○ d
        for ab in ops(a, b) {
            for abc in ops(ab, c) {
                for abcd in ops(abc, d) {
                    mark(abcd, covered);
                }
            }
        }
        // shape 2: (a ○ (b ○ c)) ○ d
        for bc in ops(b, c) {
            for abc in ops(a, bc) {
                for abcd in ops(abc, d) {
                    mark(abcd, covered);
                }
            }
        }
        // shape 3: (a ○ b) ○ (c ○ d)
        for ab in ops(a, b) {
            for cd in ops(c, d) {
                for abcd in ops(ab, cd) {
                    mark(abcd, covered);
                }
            }
        }
        // shape 4: a ○ ((b ○ c) ○ d)
        for bc in ops(b, c) {
            for bcd in ops(bc, d) {
                for abcd in ops(a, bcd) {
                    mark(abcd, covered);
                }
            }
        }
        // shape 5: a ○ (b ○ (c ○ d))
        for cd in ops(c, d) {
            for bcd in ops(b, cd) {
                for abcd in ops(a, bcd) {
                    mark(abcd, covered);
                }
            }
        }
    }
}

#[inline]
fn ops(a: f64, b: f64) -> [f64; 5] {
    [
        a + b,
        a - b,
        a * b,
        if b != 0.0 { a / b } else { f64::NAN },
        f64::NAN,
    ]
}

#[inline]
fn mark(v: f64, covered: &mut [bool; 10_000]) {
    if v.is_nan() || v <= 0.0 {
        return;
    }
    let r = v.round();
    if (v - r).abs() < 1e-9 && r < 10_000.0 {
        covered[r as usize] = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0093::solve_0093;

    #[test]
    fn test() {
        solve_print_and_check(solve_0093, 1258);
    }
}
