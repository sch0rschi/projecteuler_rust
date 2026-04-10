use projecteuler::digits::get_digits;

fn main() {
    let mut max_digits = 0;
    for i in 1.. {
        if i * 9i64.pow(5) < 10i64.pow(i as u32) {
            break;
        }
        max_digits = i;
    }

    let mut sum = 0;
    for i in 2u64..10i64.pow(max_digits as u32 + 1) as u64 {
        let digits = get_digits(i);
        let digit_sum = digits.iter().map(|x| {x.pow(5)}).sum::<u64>();
        if digit_sum == i {
            sum += i;
        }
    }

    println!("{}", sum);
}
