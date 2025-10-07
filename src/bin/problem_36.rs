use projecteuler::digits::{get_digits, get_digits_in_binary};

fn main() {
    let mut sum = 0;
    for i in 1..1_000_000 {
        let digits = get_digits(i);
        if digits == digits.iter().rev().copied().collect::<Vec<i64>>() {
            let digits_in_binary = get_digits_in_binary(i);
            if digits_in_binary == digits_in_binary.iter().rev().copied().collect::<Vec<bool>>() {
                sum += i;
            }
        }
    }
    println!("{}", sum);
}
