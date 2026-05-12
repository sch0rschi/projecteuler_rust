pub fn pentagonal(n: usize) -> usize {
    n * (3 * n - 1) / 2
}

pub fn is_pentagonal(p: usize) -> bool {
    let discriminant = 1 + 24 * p;
    let sqrt = (discriminant as f64).sqrt() as usize;
    (sqrt * sqrt == discriminant || (sqrt + 1) * (sqrt + 1) == discriminant)
        && (1 + if sqrt * sqrt == discriminant {
            sqrt
        } else {
            sqrt + 1
        }) % 6
            == 0
}
