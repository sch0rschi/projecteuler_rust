use crate::libs::primes::Primes;
use bitvec::bitvec;
use num_integer::Roots;

const LIMIT: usize = 50_000_000;


pub fn solve_0087() -> usize {
    let primes = Primes::primes_inclusive(LIMIT.sqrt() as u64);
    let primes_list = &primes.primes_list;

    // Precompute fourth powers below LIMIT
    let fourth_powers: Vec<u64> = primes_list
        .iter()
        .map(|&p| {
            let sq = p * p;
            sq * sq
        })
        .take_while(|&f| f < LIMIT as u64)
        .collect();

    // Track seen sums to avoid double-counting
    // Use a bitset for memory efficiency
    let mut seen = bitvec![0; LIMIT];

    for &p1 in primes_list.iter() {
        let square = p1 * p1;

        for &p2 in primes_list.iter() {
            let cube = p2 * p2 * p2;
            let square_cube_sum = square + cube;
            if square_cube_sum >= LIMIT as u64 {
                break;
            }

            let remaining = LIMIT as u64 - square_cube_sum - 1;

            // Binary search: find how many fourth powers fit
            let valid_count = fourth_powers.partition_point(|&f| f <= remaining);

            for &fourth in &fourth_powers[..valid_count] {
                let total = (square_cube_sum + fourth) as usize;
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
