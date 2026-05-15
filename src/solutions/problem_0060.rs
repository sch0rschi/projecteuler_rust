use crate::libs::primes::Primes;
use itertools::Itertools;

// Those two constants are "educated" guesses
const MAX_PRIME_SEARCH: usize = 10_000;
const MAX_FAST_PRIME_CHECK: usize = 100_000_000;

pub fn solve_0060() -> usize {
    let primes = Primes::primes_inclusive(MAX_FAST_PRIME_CHECK);
    let mut best: usize = usize::MAX;

    let primes_list_shortened: Vec<usize> = primes.single_iterator()
        .take_while(|&p| p <= MAX_PRIME_SEARCH)
        .filter(|&p| p == 3 || p != 2 && p != 5 && p % 3 == 1)
        .collect_vec();

    prepare_then_search(&primes, &primes_list_shortened, &mut best);

    let primes_list_shortened: Vec<usize> = primes.single_iterator()
        .take_while(|&p| p <= MAX_PRIME_SEARCH)
        .filter(|&p| p == 3 || p != 2 && p != 5 && p % 3 == 2)
        .collect_vec();

    prepare_then_search(&primes, &primes_list_shortened, &mut best);

    best
}

fn prepare_then_search(primes: &Primes, primes_list_shortened: &[usize], best: &mut usize) {
    let factors: Vec<usize> = primes_list_shortened
        .iter()
        .map(|&p| {
            let mut x = p;
            let mut factor = 10usize;

            while x >= 10 {
                x /= 10;
                factor *= 10;
            }

            factor
        })
        .collect();

    let n = primes_list_shortened.len();
    let mut cache: Vec<Option<bool>> = vec![None; n * (n - 1) / 2];

    search(
        best,
        primes,
        &mut cache,
        primes_list_shortened,
        &factors,
    );
}

#[inline(always)]
fn search(
    best: &mut usize,
    primes: &Primes,
    cache: &mut [Option<bool>],
    primes_list_shortened: &[usize],
    factors: &[usize],
) {
    #[derive(Clone, Copy)]
    struct Frame {
        next: usize,
        depth: usize,
        current_sum: usize,
        current_prime_indices: [usize; 5],
    }

    let mut stack = Vec::with_capacity(16);

    stack.push(Frame {
        next: 0,
        depth: 0,
        current_sum: 0,
        current_prime_indices: [0; 5],
    });

    while let Some(mut frame) = stack.pop() {
        if frame.depth == 5 {
            *best = frame.current_sum.min(*best);
            continue;
        }

        let remaining = 5 - frame.depth;

        while frame.next < primes_list_shortened.len() {
            let prime_1_index = frame.next;
            frame.next += 1;

            let p = primes_list_shortened[prime_1_index];

            if frame.current_sum + remaining * p >= *best {
                break;
            }

            let mut valid = true;

            for i in 0..frame.depth {
                let current_prime_index = frame.current_prime_indices[i];

                let cache_index =
                    triangular_index(prime_1_index, current_prime_index);

                let are_concat_prime = match cache[cache_index] {
                    Some(v) => v,
                    None => {
                        let v = both_concat_prime(
                            primes_list_shortened[current_prime_index],
                            p,
                            factors[current_prime_index],
                            factors[prime_1_index],
                            primes,
                        );

                        cache[cache_index] = Some(v);
                        v
                    }
                };

                if !are_concat_prime {
                    valid = false;
                    break;
                }
            }

            if !valid {
                continue;
            }

            stack.push(frame);

            let mut next_indices = frame.current_prime_indices;
            next_indices[frame.depth] = prime_1_index;

            stack.push(Frame {
                next: prime_1_index + 1,
                depth: frame.depth + 1,
                current_sum: frame.current_sum + p,
                current_prime_indices: next_indices,
            });

            break;
        }
    }
}

#[inline(always)]
fn triangular_index(i: usize, j: usize) -> usize {
    // i is always smaller than j
    i * (i - 1) / 2 + j
}

#[inline(always)]
fn both_concat_prime(a: usize, b: usize, factor_a: usize, factor_b: usize, primes: &Primes) -> bool {
    primes.is_prime(a * factor_b + b) && primes.is_prime(b * factor_a + a)
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0060::solve_0060;

    #[test]
    fn test() {
        solve_print_and_check(solve_0060, 26033);
    }
}
