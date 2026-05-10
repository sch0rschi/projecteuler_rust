use num_integer::Roots;

const LIMIT: usize = 100;
const MAX_POWER: usize = LIMIT.ilog2() as usize;
const SEEN_SIZE: usize = MAX_POWER * LIMIT + 1;


pub fn solve_0029() -> usize {
    let root = LIMIT.sqrt();

    let mut considered = [false; LIMIT + 1];
    let mut cached_distinct_terms = [0u16; MAX_POWER + 1];

    let mut seen = [0u8; SEEN_SIZE];
    let mut seen_stamp = 0u8;

    let mut count = 0;
    let mut powers_sum = 0;

    for a_1 in 2..=root {
        if considered[a_1] {
            continue;
        }

        let mut a_2 = a_1;
        let mut max_power = 0;

        for power in 1.. {
            if a_2 > LIMIT {
                max_power = power - 1;
                break;
            }

            if a_2 > root {
                powers_sum += 1;
            }
            considered[a_2] = true;
            a_2 *= a_1;
        }

        if cached_distinct_terms[max_power] != 0 {
            count += cached_distinct_terms[max_power] as usize;
        } else {
            seen_stamp += 1;
            let distinct = distinct_terms(max_power, &mut seen, seen_stamp);
            cached_distinct_terms[max_power] = distinct as u16;
            count += distinct;
        }
    }

    count += (LIMIT - root - powers_sum) * (LIMIT - 1);

    count
}

fn distinct_terms(max_power: usize, seen: &mut [u8], seen_stamp: u8) -> usize {
    let mut count = 0;
    for power in 1..=max_power {
        let base = power;
        let mut product = 2 * base;

        while product <= LIMIT * base {
            if seen[product] != seen_stamp {
                count += 1;
            }
            seen[product] = seen_stamp;
            product += base;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0029::solve_0029;

    #[test]
    fn test() {
        solve_print_and_check(solve_0029, 9183);
    }
}
