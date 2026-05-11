const FACTORIALS: [u32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

const LIMIT: usize = 2_600_000;

pub fn solve_0074() -> i32 {
    let mut chain_count = 0;
    let mut loop_length_map = vec![0u8; LIMIT];
    let mut seen = vec![false; LIMIT];
    let mut chain_list: Vec<u32> = Vec::with_capacity(64);

    let mut next_cache = vec![0u32; LIMIT];
    for n in 1..LIMIT {
        next_cache[n] = next_cache[n / 10] + FACTORIALS[n % 10];
    }

    for n in 1u32..=1_000_000 {
        chain_list.clear();

        let mut next = n;

        loop {
            if loop_length_map[next as usize] != 0 {
                let total_len = chain_list.len() as u8 + loop_length_map[next as usize];
                if total_len == 60 {
                    chain_count += 1;
                }
                break;
            }

            if seen[next as usize] {
                let pos = chain_list.iter().position(|&x| x == next).unwrap();

                let loop_length = (chain_list.len() - pos) as u8;

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

            next = next_cache[next as usize];
        }

        for &x in &chain_list {
            seen[x as usize] = false;
        }
    }

    chain_count
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0074::solve_0074;

    #[test]
    fn test() {
        solve_print_and_check(solve_0074, 402);
    }
}
