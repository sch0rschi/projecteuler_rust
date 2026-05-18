const VALID_MASK: u128 = make_valid_mask();
const CIPHER: &[u8] = &parse_cipher(include_str!("../../resources/0059_cipher.txt"));

pub fn solve_0059() -> u32 {

    let key: [u8; 3] = std::array::from_fn(|i| find_key_byte(CIPHER, i));

    CIPHER
        .iter()
        .zip(key.iter().cycle())
        .map(|(c, k)| (c ^ k) as u32)
        .sum()
}

fn find_key_byte(cipher: &[u8], offset: usize) -> u8 {
    (b'a'..=b'z')
        .find(|&k| {
            cipher
                .iter()
                .skip(offset)
                .step_by(3)
                .all(|&c| is_valid(c ^ k))
        })
        .expect("no valid key byte found")
}

#[inline(always)]
fn is_valid(c: u8) -> bool {
    c < 128 && (VALID_MASK >> c) & 1 == 1
}

const fn make_valid_mask() -> u128 {
    let mut mask = 0u128;
    let mut i = b'a';
    while i <= b'z' { mask |= 1u128 << i; i += 1; }
    let mut i = b'A';
    while i <= b'Z' { mask |= 1u128 << i; i += 1; }
    let mut i = b'0';
    while i <= b'9' { mask |= 1u128 << i; i += 1; }
    let p = b" ,.'\":;()[]+/-!?";
    let mut j = 0;
    while j < p.len() { mask |= 1u128 << p[j]; j += 1; }
    mask
}

const fn parse_cipher(s: &str) -> [u8; 1455] {
    let mut result = [0u8; 1455];
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut out = 0;
    let mut val = 0u8;

    while i <= bytes.len() {
        if i == bytes.len() || bytes[i] == b',' {
            result[out] = val;
            out += 1;
            val = 0;
        } else if bytes[i] >= b'0' && bytes[i] <= b'9' {
            val = val * 10 + (bytes[i] - b'0');
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0059::solve_0059;

    #[test]
    fn test() {
        solve_print_and_check(solve_0059, 129448);
    }
}
