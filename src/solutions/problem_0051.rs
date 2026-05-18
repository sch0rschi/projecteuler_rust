use crate::libs::primes::Primes;

const POW10: [usize; 10] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
];

const LIMIT: usize = 999_999;

#[derive(Clone, Copy, Default)]
struct DigitPositions {
    len: usize,
    pos: [usize; 6],
}

impl DigitPositions {
    fn push(&mut self, p: usize) {
        if self.len < 6 {
            self.pos[self.len] = p;
            self.len += 1;
        }
    }
}

pub fn solve_0051() -> usize {
    let primes = Primes::primes_inclusive(LIMIT);
    let mut digit_pos: [DigitPositions; 3] = [DigitPositions::default(); 3];

    for prime in primes.single_iterator() {
        if prime < 100_000 {
            continue;
        }

        fill_digit_positions(prime, &mut digit_pos);

        for (d, dp) in digit_pos.iter().enumerate() {
            if dp.len < 3 {
                continue;
            }

            for i in 0..dp.len - 2 {
                for j in i + 1..dp.len - 1 {
                    for k in j + 1..dp.len {
                        let mask = POW10[dp.pos[i]] + POW10[dp.pos[j]] + POW10[dp.pos[k]];
                        let base = prime - d * mask;

                        if let Some(smallest) = find_family_start(base, mask, &primes) {
                            return smallest;
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}

fn find_family_start(
    base: usize,
    mask: usize,
    primes: &Primes,
) -> Option<usize> {
    let mut count = 0;
    let mut first = None;

    for r in 0usize..=9 {
        if r == 0 && base < mask {
            continue;
        }
        let candidate = base + r * mask;
        if candidate >= 100_000 && primes.is_prime(candidate) {
            count += 1;
            if first.is_none() {
                first = Some(candidate);
            }
        }

        let remaining_after = 9 - r;
        if count + remaining_after < 8 {
            break;
        }
    }

    if count >= 8 { first } else { None }
}

fn fill_digit_positions(mut n: usize, out: &mut [DigitPositions; 3]) {
    *out = [DigitPositions::default(); 3];
    let mut exp = 1;
    n /= 10;

    while n > 0 {
        let digit = n % 10;
        if digit < 3 {
            out[digit].push(exp);
        }
        n /= 10;
        exp += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0051::solve_0051;

    #[test]
    fn test() {
        solve_print_and_check(solve_0051, 121313);
    }
}
