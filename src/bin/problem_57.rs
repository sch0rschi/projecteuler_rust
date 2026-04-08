use std::ops::Mul;
use std::time::Instant;
use num_bigint::BigUint;

fn main() {
    let start = Instant::now();

    let mut numinator_n = BigUint::from(7u8);
    let mut numinator_n_minus_1 = BigUint::from(3u8);

    let mut denominator_n = BigUint::from(5u8);
    let mut denominator_n_minus_1 = BigUint::from(2u8);

    let mut counter = 0;
    for i in 2..=1000 {
        if numinator_n.clone().to_string().len() > denominator_n.clone().to_string().len() {
            counter += 1;
        }
        let temp_numinator = numinator_n_minus_1.clone();
        numinator_n_minus_1 = numinator_n;
        numinator_n = numinator_n_minus_1.clone().mul(2u8) + temp_numinator;

        let temp_denuminator = denominator_n_minus_1.clone();
        denominator_n_minus_1 = denominator_n;
        denominator_n = denominator_n_minus_1.clone().mul(2u8) + temp_denuminator;
    }

    println!("{}", counter);
    println!("Elapsed: {:?}", start.elapsed());
}