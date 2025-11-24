use projecteuler::digits::{get_digits, get_number, is_pandigital};

fn main() {
    let mut max_pandigital: i64 = 0;
    for i in 1i64..10000 {
        if let Some(pandigital) = get_pandigital(i) {
            max_pandigital = max_pandigital.max(pandigital);
        }
    }
    println!("{}", max_pandigital);
}

fn get_pandigital(i: i64) -> Option<i64> {
    let mut pandigital: Vec<i64> = Vec::new();
    pandigital.reserve(15);
    for multiplier in 1..=9 {
        let add = i * multiplier;
        let mut add_digits = get_digits(add);
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
