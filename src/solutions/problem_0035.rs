use num_integer::Integer;
use smallvec::SmallVec;

const DIGITS: [usize; 4] = [9, 7, 3, 1];

const SMALLER_DIGITS: [usize; 3] = [1, 3, 7];
const SMALL_RELEVANT_PRIMES: &[usize] = &[
    3, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

pub fn solve_0035() -> usize {
    let mut rotations: SmallVec<[usize; 6]> = SmallVec::with_capacity(6);

    SMALLER_DIGITS
        .iter()
        .map(|&d| enumerate(d, 1, 1, &mut rotations, d))
        .sum::<usize>()
        + 4 // we don't count 2, 3, 5 and 7
        + 9 // neither do we count 2-digit circular primes
}

fn enumerate(
    n: usize,
    length: usize,
    pow_10: usize,
    rotations: &mut SmallVec<[usize; 6]>,
    first_digit: usize,
) -> usize {
    if length >= 6 {
        return 0;
    }

    DIGITS
        .iter()
        .take_while(|&&digit| digit >= first_digit)
        .map(|&digit| {
            let number = digit + 10 * n;
            let circular = !SMALL_RELEVANT_PRIMES
                .iter()
                .any(|&divisor| number.is_multiple_of(divisor))
                && circular_rotations(number, length + 1, pow_10 * 10, rotations);
            let longer = enumerate(number, length + 1, pow_10 * 10, rotations, first_digit);
            circular as usize * (length + 1) + longer
        })
        .sum()
}

fn circular_rotations(
    potential_circular_prime: usize,
    length: usize,
    pow_10: usize,
    rotations: &mut SmallVec<[usize; 6]>,
) -> bool {
    rotations.clear();
    let mut div = potential_circular_prime;
    let mut rem;
    for _ in 1..length {
        (div, rem) = div.div_rem(&10);
        div += rem * pow_10;
        if div < potential_circular_prime {
            return false;
        }
        rotations.push(div);
    }
    rotations.push(potential_circular_prime);
    rotations.iter().all(|&n| primal::is_prime(n as u64))
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0035::solve_0035;

    #[test]
    fn test() {
        solve_print_and_check(solve_0035, 55);
    }
}
