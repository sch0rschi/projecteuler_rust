const LIMIT: usize = 12_000;

pub fn solve_0073() -> u32 {
    let smallest_prime_factor_list = get_smalles_prime_factor_list();

    (2..=LIMIT)
        .filter_map(|d| {
            let lo = d / 3 + 1;
            let hi = (d - 1) / 2;
            if lo > hi {
                return None;
            }
            let factors = prime_factors(d, &smallest_prime_factor_list);
            Some(count_coprime(lo, hi, &factors))
        })
        .sum::<u32>()
}

fn get_smalles_prime_factor_list() -> Vec<u32> {
    let mut smallest_prime_factors = vec![0u32; LIMIT + 1];
    for i in 2..=LIMIT {
        if smallest_prime_factors[i] == 0 {
            let mut j = i;
            while j <= LIMIT {
                if smallest_prime_factors[j] == 0 {
                    smallest_prime_factors[j] = i as u32;
                }
                j += i;
            }
        }
    }
    smallest_prime_factors
}

fn prime_factors(mut n: usize, smallest_prime_factors: &[u32]) -> Vec<usize> {
    let mut factors = Vec::with_capacity(6);
    while n > 1 {
        let p = smallest_prime_factors[n] as usize;
        factors.push(p);
        while n.is_multiple_of(p) {
            n /= p;
        }
    }
    factors
}

fn count_coprime(lo: usize, hi: usize, factors: &[usize]) -> u32 {
    if lo > hi {
        return 0;
    }
    let k = factors.len();
    let mut total: i64 = 0;
    for mask in 0u32..(1 << k) {
        let mut prod = 1usize;
        let mut bits = 0u32;
        for (bit, &factor) in factors.iter().enumerate().take(k) {
            if mask & (1 << bit) != 0 {
                prod *= factor;
                bits += 1;
            }
        }
        let count = (hi / prod) as i64 - ((lo - 1) / prod) as i64;
        if bits.is_multiple_of(2) {
            total += count;
        } else {
            total -= count;
        }
    }
    total as u32
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0073::solve_0073;

    #[test]
    fn test() {
        solve_print_and_check(solve_0073, 7295372);
    }
}
