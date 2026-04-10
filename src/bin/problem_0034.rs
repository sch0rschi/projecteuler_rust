use projecteuler::digits::get_digits;
use projecteuler::factorials::get_factorial_array;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0034();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(40730, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0034() -> u64 {
    let mut sum = 0;
    let factorial_map = get_factorial_array(9);

    let mut max_length: u64 = 1;
    loop {
        let max_sum = factorial_map[9] * max_length;
        if max_sum < 10u64.pow(max_length as u32 - 1) {
            max_length -= 1;
            break;
        }
        max_length += 1;
    }

    for i in 3u64..10u64.pow(max_length as u32) {
        let digits = get_digits(i);
        let factorial_sum = digits
            .iter()
            .map(|d: &u64| -> u64 { factorial_map[*d as usize] })
            .sum::<u64>();
        if factorial_sum == i {
            sum += i;
        }
    }
    sum
}
