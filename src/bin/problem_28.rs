fn main() {
    let mut sum = 1;
    let mut last = 1;
    for i in 1..=500 {
        let new_corner = last + 2 * i;
        last = new_corner + 6 * i;
        sum += 2 * new_corner;
        sum += 2 * last;
    }
    println!("{}", sum);
}