use num_bigint::BigInt;

pub fn solve_0066() -> u64 {
    let mut period = Vec::new();

    (2..=1000)
        .filter(|&d| !is_square(d))
        .map(|d| (d, minimal_solution_x(d, &mut period)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
}

fn is_square(n: u64) -> bool {
    let r = (n as f64).sqrt() as u64;
    r * r == n
}

fn cf_sqrt_period(n: u64, in_out: &mut (u64, &mut Vec<u64>)) {
    in_out.0 = (n as f64).sqrt() as u64;
    let mut m = 0i64;
    let mut d = 1i64;
    let mut a = in_out.0 as i64;
    let target = 2 * in_out.0 as i64;
    in_out.1.clear();
    while a != target {
        m = d * a - m;
        d = (n as i64 - m * m) / d;
        a = (in_out.0 as i64 + m) / d;
        in_out.1.push(a as u64);
    }
}

enum Wide {
    Small(u128),
    Big(BigInt),
}

impl Wide {
    fn zero() -> Self {
        Wide::Small(0)
    }
    fn one() -> Self {
        Wide::Small(1)
    }

    fn add_mul(&self, a: u64, rhs: &Wide) -> Wide {
        match (self, rhs) {
            (Wide::Small(s), Wide::Small(r)) => {
                let a128 = a as u128;
                r.checked_mul(a128)
                    .and_then(|v| v.checked_add(*s))
                    .map(Wide::Small)
                    .unwrap_or_else(|| {
                        let big = BigInt::from(a) * BigInt::from(*r) + BigInt::from(*s);
                        Wide::Big(big)
                    })
            }
            (Wide::Small(s), Wide::Big(r)) => Wide::Big(BigInt::from(a) * r + BigInt::from(*s)),
            (Wide::Big(s), Wide::Small(r)) => Wide::Big(BigInt::from(a) * BigInt::from(*r) + s),
            (Wide::Big(s), Wide::Big(r)) => Wide::Big(BigInt::from(a) * r + s),
        }
    }

    fn to_bigint(self) -> BigInt {
        match self {
            Wide::Small(v) => BigInt::from(v),
            Wide::Big(b) => b,
        }
    }
}

fn minimal_solution_x(d: u64, period: &mut Vec<u64>) -> BigInt {
    let a0 = 0u64;
    cf_sqrt_period(d, &mut (a0, period));
    let period_len = period.len();
    let steps = if period_len % 2 == 0 {
        period_len
    } else {
        2 * period_len
    };

    for i in 0..steps {
        let a = period[i % period_len];
        let h_next = Wide::Small(a0 as u128).add_mul(a, &Wide::one());
        let _ = h_next;
        let _ = Wide::zero();
        let _ = Wide::one();
        break;
    }

    let mut h_prev = Wide::one();
    let mut h_curr = Wide::Small(a0 as u128);
    let mut k_prev = Wide::zero();
    let mut k_curr = Wide::one();

    for i in 0..steps {
        let a = period[i % period_len];
        let h_next = h_prev.add_mul(a, &h_curr);
        let k_next = k_prev.add_mul(a, &k_curr);
        h_prev = h_curr;
        h_curr = h_next;
        k_prev = k_curr;
        k_curr = k_next;
    }

    h_curr.to_bigint()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0066::solve_0066;

    #[test]
    fn test() {
        solve_print_and_check(solve_0066, 661);
    }
}
