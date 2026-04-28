use std::time::Instant;

const FACTORIALS: [u64; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
fn main() {
    let start = Instant::now();
    let result = solve_0074();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(402, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0074() -> u64 {
    let mut chain_count = 0;

    let mut loop_length_map = vec![0u64; 2_600_000];
    let mut seen = vec![false; 2_600_000];
    let mut chain_list: Vec<u64> = Vec::with_capacity(64);

    for n in 1..=1_000_000 {
        chain_list.clear();

        let mut next = n;

        loop {
            if loop_length_map[next as usize] != 0 {
                let total_len = chain_list.len() as u64 + loop_length_map[next as usize];
                if total_len == 60 {
                    chain_count += 1;
                }
                break;
            }

            if seen[next as usize] {
                let pos = chain_list
                    .iter()
                    .position(|&x| x == next)
                    .unwrap();

                let loop_length = (chain_list.len() - pos) as u64;

                for &loop_element in &chain_list[pos..] {
                    loop_length_map[loop_element as usize] = loop_length;
                }

                if chain_list.len() == 60 && pos > 0 {
                    chain_count += 1;
                }

                break;
            }

            seen[next as usize] = true;
            chain_list.push(next);

            next = sum_of_digit_factorials(next);
        }

        for &x in &chain_list {
            seen[x as usize] = false;
        }
    }

    chain_count
}
fn sum_of_digit_factorials(mut n: u64) -> u64 {
    let mut sum = 0;
    while n > 0 {
        sum += FACTORIALS[n as usize % 10];
        n /= 10;
    }
    sum
}
