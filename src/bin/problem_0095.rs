use std::time::Instant;
use projecteuler::divisors::ProperDivisorSums;

const LIMIT: usize = 1_000_000;

fn main() {
    let start = Instant::now();
    let result = solve_0095();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(14316, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0095() -> u32 {
    let proper_divisor_sums = ProperDivisorSums::new(LIMIT);

    let mut seen = vec![false; LIMIT + 1];
    let mut min_element = 0;
    let mut longest_chain_length = 0;
    let mut recently_seen = Vec::with_capacity(64);

    for n in 1..=LIMIT {
        if let Some((chain_length, min_chain_element)) = get_min_element_longest_chain(
            n as u32,
            &mut seen,
            longest_chain_length,
            &proper_divisor_sums,
            &mut recently_seen,
        ) {
            longest_chain_length = chain_length;
            min_element = min_chain_element;
        }
    }
    min_element
}

fn get_min_element_longest_chain(
    mut n: u32,
    seen: &mut [bool],
    longest_chain_length: usize,
    proper_divisor_sums: &ProperDivisorSums,
    recently_seen: &mut Vec<u32>,
) -> Option<(usize, u32)> {
    recently_seen.clear();
    let mut element_counter = 0;
    while n <= LIMIT as u32 && !recently_seen.contains(&n) {
        if seen[n as usize] {
            return None;
        }
        seen[n as usize] = true;
        recently_seen.push(n);
        element_counter += 1;
        n = proper_divisor_sums.get(n);
    }
    if n as usize > LIMIT || longest_chain_length > element_counter {
        return None;
    }

    let mut min_chain_element = n;
    let mut chain_length = 0;
    recently_seen.clear();
    while !recently_seen.contains(&n) {
        recently_seen.push(n);
        n = proper_divisor_sums.get(n);
        min_chain_element = min_chain_element.min(n);
        chain_length += 1;
    }

    if chain_length > longest_chain_length {
        return Some((chain_length, min_chain_element));
    }
    None
}
