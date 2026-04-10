use num_bigint::BigUint;
use num_traits::FromPrimitive;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0057();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(153, result);
    assert!(duration < std::time::Duration::from_secs(1));
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
        while n >= pow10_n { // pow10 is a quasi counter for length
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