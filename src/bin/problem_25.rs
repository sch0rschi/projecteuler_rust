use std::ops::Add;
use num_bigint::BigInt;
use num_traits::One;
fn main() {

    let mut f_p = BigInt::one();
    let mut f_n = BigInt::one();
    let mut n_th = 2;

    while f_n < BigInt::from(10).pow(999) {
        let temp = f_n.clone();
        f_n = f_n.add(f_p);
        f_p = temp;
        n_th += 1;
    }

    println!("{}", n_th);
}