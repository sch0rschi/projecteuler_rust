pub fn score(name: &str) -> i32 {
    name.bytes().map(|b| (b - b'A' + 1) as i32).sum()
}
