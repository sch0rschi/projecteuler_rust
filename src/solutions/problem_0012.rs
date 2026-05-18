pub fn solve_0012() -> u32 {
    let mut d_prev = count_divisors(1u32);

    for i in 1u32.. {
        let (d, d_next) = if i % 2 == 0 {
            let d_next = count_divisors(i + 1);
            (d_prev * d_next, count_divisors((i + 2) / 2))
        } else {
            let d_i = count_divisors(i);
            (d_i * d_prev, d_i)
        };

        if d > 500 {
            return i * (i + 1) / 2;
        }

        d_prev = d_next;
    }
    unreachable!()
}

fn count_divisors(mut n: u32) -> u32 {
    let mut total = 1u32;
    let mut count = 0;

    while n.is_multiple_of(2) {
        n /= 2;
        count += 1;
    }
    if count > 0 {
        total *= count + 1;
    }

    let mut p = 3u32;
    while p * p <= n {
        if n.is_multiple_of(p) {
            count = 0;
            while n.is_multiple_of(p) {
                n /= p;
                count += 1;
            }
            total *= count + 1;
        }
        p += 2;
    }

    if n > 1 {
        total *= 2;
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0012::solve_0012;

    #[test]
    fn test() {
        solve_print_and_check(solve_0012, 76576500);
    }
}
