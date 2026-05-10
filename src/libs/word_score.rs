pub fn score(name: &[u8]) -> u32 {
    name.iter().map(|b| (b - b'A' + 1) as u32).sum()
}
