pub fn solve_0014() -> u64 {
    let mut cache = vec![0u16; 1_000_001];
    cache[1] = 1;

    let mut max_len = 0u16;
    let mut max_val = 1u64;

    for i in (1..1_000_000u64).step_by(2) {
        let len = collatz_len(i, &mut cache);
        if len > max_len {
            max_len = len;
            max_val = i;
        }

        let even_length = cache[i.div_ceil(2) as usize];
        cache[(i + 1) as usize] = even_length;
        if even_length + 1 > max_len {
            max_len = even_length + 1;
            max_val = i + 1;
        }
    }
    max_val
}

fn collatz_len(start: u64, cache: &mut [u16]) -> u16 {
    let mut n = start;
    let mut steps = 0u16;

    while n > 1 {
        if n < cache.len() as u64
            && let cached = cache[n as usize]
            && cached != 0
        {
            steps += cached;
            break;
        }

        steps += 1;

        n = if n & 1 == 0 { n >> 1 } else { (3 * n + 1) >> 1 };
    }

    cache[start as usize] = steps;
    steps
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
