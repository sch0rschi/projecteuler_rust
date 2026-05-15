pub fn mod_pow(base: usize, mut exp: usize, modulus: usize) -> usize {
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

    result_u128 as usize
}

pub fn mod_pow_u32(base: u32, mut exp: u32, modulus: u32) -> u32 {
    let m = modulus as usize;

    let mut base = base as usize % m;
    let mut result_usize = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            result_usize = (result_usize * base) % m;
        }
        base = (base * base) % m;
        exp >>= 1;
    }

    result_usize as u32
}

