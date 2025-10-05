fn main() {
    let count: u64 = 504000;
    let mut sum: u64 = 0;

    for n in (1..=count).step_by(2) {
        sum += n * n;
    }

    println!("{}", sum);
}
