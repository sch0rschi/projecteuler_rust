use num_bigint::BigInt;

fn main() {
    let base = BigInt::from(2);
    let number = base.pow(1000);
    let sum_of_digits = number.to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).sum::<i64>();
    println!("{}", sum_of_digits)
}