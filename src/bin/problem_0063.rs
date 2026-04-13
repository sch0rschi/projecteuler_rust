use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0063();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(49, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0063() -> u8 {
    let mut count = 0;
    let mut exponent = 1;
    let mut smallest_target_power = 1;
    let mut smallest_base_tracker = 1;
    let mut previous_power = 1u128;

    loop {
        if let Some(smallest_valid_base) = find_smallest_valid_base(exponent, smallest_target_power, smallest_base_tracker, &mut previous_power) {
            count += 10 - smallest_valid_base;
            smallest_base_tracker = smallest_valid_base;
            smallest_target_power *= 10;
            exponent += 1;
        } else {
            break;
        }
    }

    count
}

fn find_smallest_valid_base(exponent: u32, smallest_target_power: u128, smallest_base: u8, previous_power: &mut u128) -> Option<u8> {
    let min_power = *previous_power * smallest_base as u128;
    if min_power >= smallest_target_power {
        *previous_power = min_power;
        return Some(smallest_base);
    }
    for i in (smallest_base +1)..=9 {
        let power = (i as u128).pow(exponent);
        if power >= smallest_target_power {
            *previous_power = power;
            return Some(i);
        }
    }
    None
}
