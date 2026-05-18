pub fn solve_0093() -> u32 {
    let mut max_n = 0;
    let mut best = 0u32;

    for a in 1u32..=6 {
        for b in a+1..=7 {
            for c in b+1..=8 {
                for d in c+1..=9 {
                    let mut covered = 0u128;
                    eval_all([a,b,c,d], &mut covered);
                    let n = (1..=100).find(|&i| covered & (1u128 << i) == 0).unwrap_or(101) - 1;
                    if n > max_n {
                        max_n = n;
                        best = a*1000 + b*100 + c*10 + d;
                    }
                }
            }
        }
    }
    best
}

type R = (i64, i64);

#[inline] fn add(a:R,b:R)->R{ if a.1==0||b.1==0{(0,0)}else{(a.0*b.1+b.0*a.1, a.1*b.1)} }
#[inline] fn sub(a:R,b:R)->R{ if a.1==0||b.1==0{(0,0)}else{(a.0*b.1-b.0*a.1, a.1*b.1)} }
#[inline] fn mul(a:R,b:R)->R{ if a.1==0||b.1==0{(0,0)}else{(a.0*b.0, a.1*b.1)} }
#[inline] fn div(a:R,b:R)->R{ if a.1==0||b.0==0{(0,0)}else{(a.0*b.1, a.1*b.0)} }

fn apply_ops(a: R, b: R, mut f: impl FnMut(R)) {
    f(add(a,b)); f(sub(a,b)); f(mul(a,b)); f(div(a,b));
}

fn mark(v: R, covered: &mut u128) {
    if v.1 == 0 || v.0 <= 0 { return; }
    if v.0 % v.1 == 0 {
        let n = (v.0 / v.1) as usize;
        if n <= 100 { *covered |= 1u128 << n; }
    }
}

fn eval_all(digits: [u32;4], covered: &mut u128) {
    let perms = [
        [0,1,2,3],[0,1,3,2],[0,2,1,3],[0,2,3,1],[0,3,1,2],[0,3,2,1],
        [1,0,2,3],[1,0,3,2],[1,2,0,3],[1,2,3,0],[1,3,0,2],[1,3,2,0],
        [2,0,1,3],[2,0,3,1],[2,1,0,3],[2,1,3,0],[2,3,0,1],[2,3,1,0],
        [3,0,1,2],[3,0,2,1],[3,1,0,2],[3,1,2,0],[3,2,0,1],[3,2,1,0],
    ];
    for p in &perms {
        let [a,b,c,d] = [
            (digits[p[0]] as i64, 1i64),
            (digits[p[1]] as i64, 1i64),
            (digits[p[2]] as i64, 1i64),
            (digits[p[3]] as i64, 1i64),
        ];
        // shape 1: ((a○b)○c)○d
        apply_ops(a,b,|ab| apply_ops(ab,c,|abc| apply_ops(abc,d,|r| mark(r,covered))));
        // shape 2: (a○(b○c))○d
        apply_ops(b,c,|bc| apply_ops(a,bc,|abc| apply_ops(abc,d,|r| mark(r,covered))));
        // shape 3: (a○b)○(c○d)
        apply_ops(a,b,|ab| apply_ops(c,d,|cd| apply_ops(ab,cd,|r| mark(r,covered))));
        // shape 4: a○((b○c)○d)
        apply_ops(b,c,|bc| apply_ops(bc,d,|bcd| apply_ops(a,bcd,|r| mark(r,covered))));
        // shape 5: a○(b○(c○d))
        apply_ops(c,d,|cd| apply_ops(b,cd,|bcd| apply_ops(a,bcd,|r| mark(r,covered))));
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
