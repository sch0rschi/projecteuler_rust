use crate::libs::primes::Primes;
use bitvec::bitvec;
use itertools::Itertools;
use num_integer::Roots;

const LIMIT: usize = 50_000_000;


pub fn solve_0087() -> usize {
    let primes = Primes::primes_inclusive(LIMIT.sqrt());
    let primes_list = primes.get_primes_list();

    // Precompute fourth powers below LIMIT
    let fourth_powers = primes_list
        .iter()
        .map(|&p| {
            let sq = p * p;
            sq * sq
        })
        .take_while(|&f| f < LIMIT)
        .collect_vec();

    let mut seen = bitvec![0; LIMIT];

    for &p1 in primes_list.iter() {
        let square = p1 * p1;

        for &p2 in primes_list.iter() {
            let cube = p2 * p2 * p2;
            let square_cube_sum = square + cube;
            if square_cube_sum >= LIMIT {
                break;
            }

            let remaining = LIMIT - square_cube_sum - 1;

            let valid_count = fourth_powers.partition_point(|&f| f <= remaining);

            for &fourth in &fourth_powers[..valid_count] {
                let total = square_cube_sum + fourth;
                seen.set(total, true);
            }
        }
    }

    seen.count_ones()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0087::solve_0087;

    #[test]
    fn test() {
        solve_print_and_check(solve_0087, 1097343);
    }
}
