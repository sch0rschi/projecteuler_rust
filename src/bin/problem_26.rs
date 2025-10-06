fn main() {
    let mut d = 1;
    let mut max_fraction_cycle_length = 0;
    for i in 2..1000 {
        let fraction_cycle_length = get_fraction_cycle_length(i);
        if fraction_cycle_length > max_fraction_cycle_length {
            max_fraction_cycle_length = fraction_cycle_length;
            d = i;
        }
    }
    println!("{}", d);
}

fn get_fraction_cycle_length(n: i32) -> i32 {
    let mut remainder = 1;
    let mut occurrence: i32 = 0;
    let mut remainder_occurrences: Vec<i32> = vec![0; n as usize];
    remainder_occurrences.resize(n as usize, 0);
    while remainder > 0 {
        remainder *= 10;
        let fraction = remainder / n;
        remainder -= n * fraction;
        if remainder_occurrences[remainder as usize] != 0 {
            return occurrence - remainder_occurrences[remainder as usize];
        }
        remainder_occurrences[remainder as usize] = occurrence;
        occurrence += 1;
    }
    0
}
