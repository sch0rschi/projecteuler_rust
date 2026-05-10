pub fn get_continued_fraction_sequence(n: u64) -> Vec<u64> {
    let a0 = (n as f64).sqrt() as u64;
    let mut sequence = vec![a0];

    if a0 * a0 == n {
        return sequence;
    }

    let mut m = 0;
    let mut d = 1;
    let mut a = a0;

    loop {
        m = d * a - m;
        d = (n - m * m) / d;
        a = (a0 + m) / d;

        sequence.push(a);

        if a == 2 * a0 {
            break;
        }
    }

    sequence
}
