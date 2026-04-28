use projecteuler::evaluation_helper::solve_print_and_check;
use std::collections::HashMap;

fn main() {
    solve_print_and_check(solve_0012, 76576500);
}

fn solve_0012() -> u32 {
    let mut cache: HashMap<u32, u32> = HashMap::new();

    (1..)
        .find_map(|i: u32| {
            let (a, b) = if i.is_multiple_of(2) {
                (i / 2, i + 1)
            } else {
                (i, i.div_ceil(2))
            };

            let da = *cache.entry(a).or_insert_with(|| count_divisors(a));
            let db = *cache.entry(b).or_insert_with(|| count_divisors(b));

            if da * db > 500 {
                Some(i * (i + 1) / 2)
            } else {
                None
            }
        })
        .unwrap()
}

fn count_divisors(mut n: u32) -> u32 {
    let mut total = 1u32;
    let mut count= 0;

    while n.is_multiple_of(2) {
        n /= 2;
        count += 1;
    }
    if count > 0 {
        total *= count + 1;
    }

    let mut p = 3;
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
