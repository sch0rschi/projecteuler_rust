use projecteuler::digits::get_digits;
use projecteuler::factorials::get_factorial_array;

fn main() {
    let mut sum = 0;
    let factorial_map = get_factorial_array(9);

    let mut max_length = 1;
    loop {
        let max_sum = factorial_map[9] * max_length;
        if max_sum < 10i64.pow(max_length as u32-1) {
            max_length -= 1;
            break;
        }
        max_length += 1;
    }

    for i in 3..10i64.pow(max_length as u32) {
        let digits = get_digits(i);
        let factorial_sum = digits
            .iter()
            .map(|d: &i64| -> i64 { factorial_map[*d as usize] })
            .sum::<i64>();
        if factorial_sum == i {
            sum += i;
        }
    }
    println!("{}", sum);
}
