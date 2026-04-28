use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0032, 45228);
}

fn solve_0032() -> i64 {
    let mut used = [false; 10000];
    let mut sum = 0i64;

    for a in 2..10 {
        for b in 1234..9877 {
            let c = a * b;
            if c >= 10000 {
                continue;
            }

            if is_pandigital(a, b, c) && !used[c] {
                used[c] = true;
                sum += c as i64;
            }
        }
    }

    for a in 12..100 {
        for b in 123..1000 {
            let c = a * b;
            if c >= 10000 {
                continue;
            }

            if is_pandigital(a, b, c) && !used[c] {
                used[c] = true;
                sum += c as i64;
            }
        }
    }

    sum
}

#[inline(always)]
fn is_pandigital(a: usize, b: usize, c: usize) -> bool {
    let mut mask: u32 = 0;
    let mut count = 0;

    let push = |mut x: usize, mask: &mut u32, count: &mut usize| {
        while x > 0 {
            let d = x % 10;
            if d == 0 {
                return false;
            }
            let bit = 1 << d;
            if (*mask & bit) != 0 {
                return false;
            }
            *mask |= bit;
            *count += 1;
            x /= 10;
        }
        true
    };

    if !push(a, &mut mask, &mut count) {
        return false;
    }
    if !push(b, &mut mask, &mut count) {
        return false;
    }
    if !push(c, &mut mask, &mut count) {
        return false;
    }

    count == 9 && mask == 0b1111111110 // bits 1..9 set
}
