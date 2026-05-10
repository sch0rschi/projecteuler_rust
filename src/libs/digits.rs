pub fn get_digits(n: u64) -> Vec<u64> {
    let mut digits: Vec<u64> = Vec::new();
    let mut number = n;
    while number > 0 {
        digits.push(number % 10);
        number /= 10;
    }
    digits
}

pub fn is_pandigital(digits: &[u64]) -> bool {
    for digit in 1..9 {
        if !digits.contains(&digit) {
            return false;
        }
    }
    true
}

pub fn get_number(digits: &[u64]) -> u64 {
    let mut number = 0;
    for digit in digits.iter().rev() {
        number *= 10;
        number += *digit;
    }
    number
}

pub fn get_digits_in_binary(n: u64) -> Vec<bool> {
    let mut digits: Vec<bool> = Vec::new();
    let mut number = n;
    while number > 0 {
        digits.push(number % 2 == 1);
        number /= 2;
    }
    digits
}

pub fn get_digit_count_encoding_15_max(mut n: u64) -> u64 {
    let mut encoding = 0;
    while n > 0 {
        encoding += 1 << (4 * (n % 10));
        n /= 10;
    }
    encoding
}
