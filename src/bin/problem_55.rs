use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut count = 0;

    for i in 1u128..10000 {
        if is_lychrel(i) {
            count += 1;
        }
    }

    println!("{}", count);
    println!("Elapsed: {:?}", start.elapsed());
}

fn is_lychrel(mut n: u128) -> bool {
    for _ in 1..=50 {
        n += reverse(n);
        if n == reverse(n) {
            return false;
        }
    }
    true
}

fn reverse(mut n: u128) -> u128 {
    let mut result = 0u128;
    while n > 0 {
        result = result * 10 + n % 10;
        n /= 10;
    }
    result
}
