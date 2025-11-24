pub fn get_digits(n: i64) -> Vec<i64> {
    let mut digits: Vec<i64> = Vec::new();
    let mut number = n;
    while number > 0 {
        digits.push(number % 10);
        number /= 10;
    }
    digits.reverse();
    digits
}

pub fn is_pandigital(digits: &[i64]) -> bool {
    for digit in 1 .. 9 {
        if !digits.contains(&digit) {
            return false;
        }
    }
    true
}

pub fn get_number(digits: &[i64]) -> i64 {
    let mut number = 0;
    for digit in digits.iter().rev() {
        number *= 10;
        number += *digit;
    }
    number
}

pub fn get_digits_in_binary(n: i64) -> Vec<bool> {
    let mut digits: Vec<bool> = Vec::new();
    let mut number = n;
    while number > 0 {
        digits.push(number % 2 == 1);
        number /= 2;
    }
    digits
}
