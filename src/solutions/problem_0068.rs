pub fn solve_0068() -> u64 {
    let mut max = 0u64;

    for_each_inner_ring(|inner, used| {
        if let Some(outer) = derive_outer_ring(inner, used)
            && is_canonical(&outer)
            && contains_ten(&outer)
        {
            max = max.max(concatenate(inner, &outer));
        }
    });

    max
}

fn for_each_inner_ring(mut f: impl FnMut(&[usize; 5], u16)) {
    for a in 1usize..=9 {
        let used_a = bit(a);
        for b in 1..=9 {
            if used_a & bit(b) != 0 {
                continue;
            }
            let used_b = used_a | bit(b);
            for c in 1..=9 {
                if used_b & bit(c) != 0 {
                    continue;
                }
                let used_c = used_b | bit(c);
                for d in 1..=9 {
                    if used_c & bit(d) != 0 {
                        continue;
                    }
                    let used_d = used_c | bit(d);
                    let partial = a + b + c + d;
                    // line sum s = (55 + inner_sum) / 5, so inner_sum must be divisible by 5
                    let e_mod = (5 - partial % 5) % 5;
                    for e in (e_mod..=9).step_by(5) {
                        if e == 0 || used_d & bit(e) != 0 {
                            continue;
                        }
                        f(&[a, b, c, d, e], used_d | bit(e));
                    }
                }
            }
        }
    }
}

fn derive_outer_ring(inner: &[usize; 5], used: u16) -> Option<[usize; 5]> {
    let s = (55 + inner.iter().sum::<usize>()) / 5;
    let mut outer = [0usize; 5];
    let mut outer_used: u16 = 0;

    for i in 0..5 {
        let oi = s.checked_sub(inner[i] + inner[(i + 1) % 5])?;
        if oi == 0 || oi > 10 || (used | outer_used) & bit(oi) != 0 {
            return None;
        }
        outer[i] = oi;
        outer_used |= bit(oi);
    }

    Some(outer)
}

fn is_canonical(outer: &[usize; 5]) -> bool {
    outer.iter().min() == Some(&outer[0])
}

fn contains_ten(outer: &[usize; 5]) -> bool {
    outer.contains(&10)
}

fn concatenate(inner: &[usize; 5], outer: &[usize; 5]) -> u64 {
    let mut result = 0u64;
    for i in 0..5 {
        for &n in &[outer[i], inner[i], inner[(i + 1) % 5]] {
            result = if n == 10 {
                result * 100 + 10
            } else {
                result * 10 + n as u64
            };
        }
    }
    result
}

#[inline(always)]
fn bit(n: usize) -> u16 {
    1 << n
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0068::solve_0068;

    #[test]
    fn test() {
        solve_print_and_check(solve_0068, 6531031914842725);
    }
}
