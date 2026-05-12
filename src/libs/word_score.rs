pub fn score(name: &str) -> u32 {
    let mut sum = 0u32;

    for b in name.as_bytes() {
        sum += (b - b'A' + 1) as u32;
    }

    sum
}