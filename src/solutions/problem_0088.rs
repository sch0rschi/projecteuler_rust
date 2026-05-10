use crate::libs::multiset_partitions::MultisetPartitions;
use crate::libs::primes::Primes;
use itertools::Itertools;

const LIMIT: usize = 12_000;


pub fn solve_0088() -> u64 {
    let primes = Primes::primes_inclusive(2 * LIMIT as u64);
    let mut min_product_sum: [u64; LIMIT + 1] = [0u64; LIMIT + 1];
    let mut count = 1;
    for i in 4u64.. {
        let prime_factorization = primes.prime_factors(i);

        let multiset_partitions = MultisetPartitions::new(prime_factorization);
        for multiset_partition in multiset_partitions {
            if multiset_partition.blocks.len() == 1 {
                continue;
            }
            let product_sum: usize = multiset_partition
                .blocks
                .iter()
                .map(|block: &Vec<u64>| block.iter().product::<u64>() as usize)
                .sum();
            let k = multiset_partition.blocks.len() + i as usize - product_sum;
            if k <= LIMIT && min_product_sum[k] == 0 {
                count += 1;
                min_product_sum[k] = i;
                if count == LIMIT {
                    return min_product_sum.iter().unique().sum();
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0088::solve_0088;

    #[test]
    fn test() {
        solve_print_and_check(solve_0088, 7587457);
    }
}
