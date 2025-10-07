pub fn get_digits(n: i64) -> Vec<i64> {
    let mut digits: Vec<i64> = Vec::new();
    let mut number = n;
    while number > 0 {
        digits.push(number % 10);
        number /= 10;
    }
    digits
}

pub fn get_number(digits: &[i64]) -> i64 {
    let mut number = 0;
    for digit in digits.iter().rev() {
        number *= 10;
        number += *digit;
    }
    number
}
