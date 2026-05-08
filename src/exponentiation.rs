pub fn mod_pow(base: u64, mut exp: u64, modulus: u64) -> u64 {
    let m = modulus as u128;

    let mut base = base as u128 % m;
    let mut result_u128 = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            result_u128 = (result_u128 * base) % m;
        }
        base = (base * base) % m;
        exp >>= 1;
    }

    result_u128 as u64
}

pub fn mod_pow_u32(base: u32, mut exp: u32, modulus: u32) -> u32 {
    let m = modulus as u64;

    let mut base = base as u64 % m;
    let mut result_u64 = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            result_u64 = (result_u64 * base) % m;
        }
        base = (base * base) % m;
        exp >>= 1;
    }

    result_u64 as u32
}

