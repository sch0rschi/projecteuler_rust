use crate::libs::primes::Primes;
use num_integer::Roots;

const LIMIT: usize = 50_000_000;

pub fn solve_0087() -> usize {
    let primes = Primes::primes_inclusive(LIMIT.sqrt());
    let primes_list = primes.get_primes_list();

    let squares: Vec<usize> = primes_list.iter().map(|&p| p * p).take_while(|&x| x < LIMIT).collect();
    let cubes:   Vec<usize> = primes_list.iter().map(|&p| p * p * p).take_while(|&x| x < LIMIT).collect();
    let fourths: Vec<usize> = primes_list.iter().map(|&p| { let sq = p*p; sq*sq }).take_while(|&x| x < LIMIT).collect();

    let mut seen = vec![0u64; LIMIT.div_ceil(64)];

    for &f in &fourths {
        for &c in &cubes {
            let fc = f + c;
            if fc >= LIMIT { break; }
            for &s in &squares {
                let total = fc + s;
                if total >= LIMIT { break; }
                seen[total >> 6] |= 1u64 << (total & 63);
            }
        }
    }

    seen.iter().map(|w| w.count_ones() as usize).sum()
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
