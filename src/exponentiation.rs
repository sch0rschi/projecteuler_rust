pub fn mod_pow(base: u64, mut exp: usize, modulus: u64) -> u64 {
    let m = modulus as u128;

    let mut base = base as u128 % m;
    let mut result128 = 1u128;

    while exp > 0 {
        if exp & 1 == 1 {
            result128 = (result128 * base) % m;
        }
        base = (base * base) % m;
        exp >>= 1;
    }

    result128 as u64
}