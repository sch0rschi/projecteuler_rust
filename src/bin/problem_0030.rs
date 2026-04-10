use projecteuler::digits::get_digits;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0030();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(443839, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0030() -> u64 {
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
        let digit_sum = digits.iter().map(|x| { x.pow(5) }).sum::<u64>();
        if digit_sum == i {
            sum += i;
        }
    }

    sum
}
