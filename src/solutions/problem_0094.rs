const LIMIT: i64 = 1_000_000_000;

pub fn solve_0094() -> i64 {
    // We seek triangles with sides (a, a, a±1) and integer area.
    //
    // Applying Heron's formula to (a, a, c) with s = (2a+c)/2:
    //   16·Area² = (2a+c)·c²·(2a-c) ... wait, let's be precise:
    //   16·Area² = (a+b+c)(-a+b+c)(a-b+c)(a+b-c)
    //            = (2a+c)(c)(c)(2a-c)      [since a=b]
    //            = c²(4a²-c²)
    //
    // Type A: c = a+1
    //   16·Area² = (a+1)²(4a²-(a+1)²) = (a+1)²(3a²-2a-1) = (a+1)²(3a+1)(a-1)
    //   For integer area, (a+1)²(3a+1)(a-1) must be a perfect square.
    //   Since (a+1)² is already square, need (3a+1)(a-1) = k²
    //   Substituting m = 2a (even): reduces to Pell equation x²-3y²=1
    //   Solutions: a = 5, 65, 901, 12545, ...
    //   Recurrence derived from Pell: a_next = 14·a_cur - a_prev - 4
    //   Virtual seed (prev=1) satisfies: 14·1 - ? - 4 = 5 → consistent
    //
    // Type B: c = a-1
    //   16·Area² = (a-1)²(4a²-(a-1)²) = (a-1)²(3a²+2a-1) = (a-1)²(3a-1)(a+1)
    //   For integer area, need (3a-1)(a+1) = k²
    //   Also reduces to Pell equation x²-3y²=1, different branch
    //   Solutions: a = 17, 241, 3361, 46817, ...
    //   Recurrence: a_next = 14·a_cur - a_prev + 4
    //   Virtual seed (prev=1) satisfies: 14·1 - ? + 4 = 17 → consistent
    //
    // Perimeters are 3a+1 (Type A) and 3a-1 (Type B), both must be ≤ LIMIT.

    let mut sum = 0i64;

    let (mut prev, mut cur) = (1i64, 5i64);
    while 3 * cur < LIMIT {
        sum += 3 * cur + 1;
        (prev, cur) = (cur, 14 * cur - prev - 4);
    }

    let (mut prev, mut cur) = (1i64, 17i64);
    while 3 * cur - 1 <= LIMIT {
        sum += 3 * cur - 1;
        (prev, cur) = (cur, 14 * cur - prev + 4);
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0094::solve_0094;

    #[test]
    fn test() {
        solve_print_and_check(solve_0094, 518408346);
    }
}
