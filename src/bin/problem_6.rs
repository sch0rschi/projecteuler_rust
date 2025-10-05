fn main() {
    let mut squares_sum = 0;
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
        squares_sum += i * i;
    }
    println!("{}", sum * sum - squares_sum);
}
