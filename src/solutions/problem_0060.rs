use crate::libs::primes::Primes;
use itertools::Itertools;

// This constant is an "educated" guesses
const MAX_PRIME_SEARCH: usize = 10_000;

#[derive(Clone, Copy)]
struct Frame {
    depth: usize,
    sum: usize,
    gathered_node_indices: [usize; 5],
}

pub fn solve_0060() -> usize {
    let primes = Primes::primes_inclusive(MAX_PRIME_SEARCH * MAX_PRIME_SEARCH);
    let mut best = usize::MAX;

    prepare_then_search(&primes, &mut best, 1);
    prepare_then_search(&primes, &mut best, 2);

    best
}

fn prepare_then_search(primes: &Primes, best: &mut usize, congruent: usize) {
    let primes_list: Vec<usize> = primes
        .single_iterator()
        .take_while(|&p| p <= MAX_PRIME_SEARCH)
        .filter(|&p| p == 3 || (p != 2 && p != 5 && p % 3 == congruent))
        .collect_vec();

    let n = primes_list.len();
    let mut both_concat_prime_map = vec![false; n * (n - 1) / 2];

    let mut factors = Vec::with_capacity(primes_list.len());
    let mut threshold = 10;
    for &p in &primes_list {
        while threshold <= p {
            threshold *= 10;
        }
        factors.push(threshold);
    }
    for (i2, &p2) in primes_list.iter().enumerate() {
        let factor_p2 = factors[i2];
        for (i1, &p1) in primes_list.iter().enumerate().take(i2) {
            both_concat_prime_map[triangular_index(i1, i2)] =
                both_concat_prime(p1, p2, factors[i1], factor_p2, primes);
        }
    }

    search(&primes_list, &both_concat_prime_map, best);
}

#[inline(always)]
fn search(primes: &[usize], both_concat_prime_map: &[bool], best: &mut usize) {
    let mut stack: Vec<Frame> = Vec::new();

    for (i, &p) in primes.iter().enumerate().rev().skip(4) {
        stack.push(Frame {
            depth: 1,
            sum: p,
            gathered_node_indices: [i; 5],
        })
    }

    while let Some(frame) = stack.pop() {
        let Frame {
            depth: current_depth,
            sum: current_sum,
            gathered_node_indices,
        } = frame;

        if current_depth == 5 {
            *best = current_sum.min(*best);
            continue;
        }

        let remaining = 5 - current_depth;
        let start = gathered_node_indices[current_depth - 1] + 1;

        for potential_index in (start..primes.len() - 4 + current_depth).rev() {
            let potential_prime = primes[potential_index];

            if current_sum + remaining * potential_prime >= *best {
                break;
            }

            let all_both_concatenate_prime =
                gathered_node_indices[0..current_depth]
                    .iter()
                    .all(|&gathered_node_index| {
                        both_concat_prime_map
                            [triangular_index(gathered_node_index, potential_index)]
                    });

            if !all_both_concatenate_prime {
                continue;
            }

            let mut new_frame = frame;
            new_frame.depth = current_depth + 1;
            new_frame.sum = current_sum + potential_prime;
            new_frame.gathered_node_indices[current_depth] = potential_index;

            stack.push(new_frame);
        }
    }
}

fn triangular_index(i: usize, j: usize) -> usize {
    j * (j - 1) / 2 + i
}

fn both_concat_prime(
    a: usize,
    b: usize,
    factor_a: usize,
    factor_b: usize,
    primes: &Primes,
) -> bool {
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
