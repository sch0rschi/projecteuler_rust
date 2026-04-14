use num_bigint::BigInt;
use num_traits::{One, Zero};

pub fn get_min_x(continued_fraction_sequence: &[u64], n: u64) -> BigInt {
    let period = &continued_fraction_sequence[1..];

    if period.is_empty() {
        return BigInt::one();
    }

    let a0 = continued_fraction_sequence[0];

    let mut p_prev2 = BigInt::one();
    let mut p_prev1 = BigInt::from(a0);

    let mut q_prev2 = BigInt::zero();
    let mut q_prev1 = BigInt::one();

    let lhs = &p_prev1 * &p_prev1 - BigInt::from(n) * &q_prev1 * &q_prev1;
    if lhs == BigInt::one() {
        return p_prev1.clone();
    }

    let mut i = 1usize;

    loop {
        let a = BigInt::from(period[(i - 1) % period.len()]);

        let p = &a * &p_prev1 + &p_prev2;
        let q = &a * &q_prev1 + &q_prev2;

        let lhs = &p * &p - BigInt::from(n) * &q * &q;
        if lhs == BigInt::one() {
            return p;
        }

        p_prev2 = p_prev1;
        p_prev1 = p;

        q_prev2 = q_prev1;
        q_prev1 = q;

        i += 1;
    }
}
