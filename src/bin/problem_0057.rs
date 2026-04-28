use num_bigint::BigUint;
use num_traits::FromPrimitive;
use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0057, 153);
}

fn solve_0057() -> i32 {
    let mut n = BigUint::from_u8(7).unwrap();
    let mut n_prev = BigUint::from_u8(3).unwrap();

    let mut d = BigUint::from_u8(5).unwrap();
    let mut d_prev = BigUint::from_u8(2).unwrap();

    let mut counter = 0;

    let mut pow10_n = BigUint::from_u8(10).unwrap();
    let mut pow10_d = BigUint::from_u8(10).unwrap();

    for _ in 2..=1000 {
        while n >= pow10_n {
            // pow10 is a quasi counter for length
            pow10_n *= 10u8;
        }
        while d >= pow10_d {
            pow10_d *= 10u8;
        }

        if pow10_n > pow10_d {
            counter += 1;
        }

        let new_n = (&n << 1) + &n_prev;
        n_prev = n;
        n = new_n;

        let new_d = (&d << 1) + &d_prev;
        d_prev = d;
        d = new_d;
    }

    counter
}
