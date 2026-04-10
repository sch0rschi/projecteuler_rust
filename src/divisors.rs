pub fn proper_divisor_sum(n: u64) -> u64 {
    proper_divisors(n).iter().sum()
}

fn proper_divisors(n: u64) -> Vec<u64> {
    let mut n_sqrt = n.isqrt();
    let mut proper_divisors: Vec<u64> = Vec::new();
    proper_divisors.push(1);
    if n == 1 {
        return proper_divisors;
    } else if n_sqrt * n_sqrt == n {
        proper_divisors.push(n_sqrt);
    } else {
        n_sqrt += 1;
    }

    for divisor_candidate in 2..n_sqrt {
        if n.is_multiple_of(divisor_candidate) {
            proper_divisors.push(divisor_candidate);
            proper_divisors.push(n / divisor_candidate);
        }
    }
    proper_divisors
}
