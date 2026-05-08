use num_integer::Integer;
use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::primes_u32::is_prime;
use smallvec::SmallVec;

const DIGITS: [u32; 4] = [9, 7, 3, 1];
const SMALLER_DIGITS: [u32; 3] = [1, 3, 7];

const SMALL_RELEVANT_PRIMES: &[u32] = &[3, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

fn main() {
    solve_print_and_check(solve_0035, 55);
}

fn solve_0035() -> u32 {
    let mut rotations: SmallVec<[u32; 6]> = SmallVec::with_capacity(6);

    SMALLER_DIGITS
        .iter()
        .map(|&d| enumerate(d, 1, 1, &mut rotations, d))
        .sum::<u32>()
        + 4 // we don't count 2, 3, 5 and 7
        + 9 // neither do we count 2-digit circular primes
}

fn enumerate(
    n: u32,
    length: u32,
    pow_10: u32,
    rotations: &mut SmallVec<[u32; 6]>,
    first_digit: u32,
) -> u32 {
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
            let longer = enumerate(
                number,
                length + 1,
                pow_10 * 10,
                rotations,
                first_digit,
            );
            circular as u32 * (length + 1) + longer
        })
        .sum()
}

fn circular_rotations(
    potential_circular_prime: u32,
    length: u32,
    pow_10: u32,
    rotations: &mut SmallVec<[u32; 6]>,
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
    rotations.iter().all(|&n| is_prime(n))
}
