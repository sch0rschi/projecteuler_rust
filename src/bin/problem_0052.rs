use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0052, 142857);
}

fn solve_0052() -> i32 {
    let mut from = 10000;

    loop {
        from *= 10;
        let end = from * 10 / 6;

        for i in (from + 8..=end).step_by(9) {
            let base = get_digit_count_encoded(i);

            if get_digit_count_encoded(2 * i) != base {
                continue;
            }

            if get_digit_count_encoded(3 * i) != base {
                continue;
            }

            if get_digit_count_encoded(4 * i) != base {
                continue;
            }

            if get_digit_count_encoded(5 * i) != base {
                continue;
            }

            if get_digit_count_encoded(6 * i) != base {
                continue;
            }

            return i;
        }
    }
}

// 6 bits per digit allow the count up to 63
fn get_digit_count_encoded(mut n: i32) -> u64 {
    let mut sig = 0u64;
    while n > 0 {
        let digit = n % 10;
        sig += 1 << (digit * 6);
        n /= 10;
    }
    sig
}
