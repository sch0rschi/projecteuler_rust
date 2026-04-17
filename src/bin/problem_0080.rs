use num_bigint::BigInt;
use std::time::Instant;
use num_integer::Roots;

fn main() {
    let start = Instant::now();
    let result = solve_0080();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(40886, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0080() -> u64 {
    (2..=100)
        .filter(|&i| {
            let sqrt = i.sqrt();
            sqrt * sqrt != i
        })
        .map(decimal_digit_sum_100)
        .sum()
}

fn decimal_digit_sum_100(n: u64) -> u64 {
    let ten = BigInt::from(10u32);
    let hundred = BigInt::from(100u32);

    let mut sqrt_scale = BigInt::from(n.isqrt());
    let mut n_scale = BigInt::from(n);

    for _ in 0..=100 {
        sqrt_scale *= &ten;
        n_scale *= &hundred;

        let digit = best_digit(&sqrt_scale, &n_scale);

        sqrt_scale += digit;
    }

    sqrt_scale
        .to_string()
        .chars()
        .take(100)
        .map(|c| (c as u8 - b'0') as u64)
        .sum()
}

fn best_digit(sqrt_scale: &BigInt, n_scale: &BigInt) -> u32 {
    let mut low = 0i32;
    let mut high = 9i32;
    let mut best = 0u32;

    while low <= high {
        let mid = (low + high) / 2;

        let candidate = sqrt_scale + mid;
        let square = &candidate * &candidate;

        if square <= *n_scale {
            best = mid as u32;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    best
}
