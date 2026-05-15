pub fn get_digit_count_encoding_15_max(mut n: u64) -> u64 {
    let mut encoding = 0;
    while n > 0 {
        encoding += 1 << (4 * (n % 10));
        n /= 10;
    }
    encoding
}

pub fn get_digit_occurrence_mask(n: u64, length: usize) -> u16 {
    let mut mask: u16 = 0;
    let mut number = n;
    for _ in 0..length {
        mask |= 1 << (number % 10);
        number /= 10;
    }
    mask
}