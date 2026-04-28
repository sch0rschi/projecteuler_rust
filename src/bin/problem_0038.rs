use projecteuler::digits::{get_digits, get_number, is_pandigital};
use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0038, 932718654);
}

fn solve_0038() -> u64 {
    let mut max_pandigital: u64 = 0;
    for i in 1u64..10000 {
        if let Some(pandigital) = get_pandigital(i) {
            max_pandigital = max_pandigital.max(pandigital);
        }
    }
    max_pandigital
}

fn get_pandigital(i: u64) -> Option<u64> {
    let mut pandigital: Vec<u64> = Vec::with_capacity(15);
    for multiplier in 1..=9 {
        let add = i * multiplier;
        let mut add_digits = get_digits(add);
        add_digits.reverse();
        pandigital.append(&mut add_digits);
        if pandigital.len() >= 9 {
            break;
        }
    }
    if pandigital.len() == 9 && is_pandigital(&pandigital) {
        pandigital.reverse();
        Some(get_number(&pandigital))
    } else {
        None
    }
}
