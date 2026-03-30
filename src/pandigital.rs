pub fn is_1_to_length_pandigital(n: i64) -> bool {
    if n <= 0 {
        return false;
    }

    let mut digits = [false; 10];
    let mut count = 0;
    let mut num = n;

    while num > 0 {
        let digit = (num % 10) as usize;
        if digit == 0 || digits[digit] {
            return false;
        }
        digits[digit] = true;
        num /= 10;
        count += 1;
    }

    for i in 1..=count {
        if !digits[i] {
            return false;
        }
    }

    true
}