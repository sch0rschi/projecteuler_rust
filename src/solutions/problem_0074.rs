const FACTORIALS: [u32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
const LIMIT: u32 = 1_000_000;
const EXTENDING_LIMIT: usize = 2_600_000;

pub fn solve_0074() -> i32 {
    let mut chain_count = 0;
    let mut loop_length_map = vec![0u8; EXTENDING_LIMIT];
    let mut chain_list: Vec<u32> = Vec::with_capacity(64);

    let mut next_cache = vec![0u32; EXTENDING_LIMIT];
    for n in 1..EXTENDING_LIMIT {
        next_cache[n] = next_cache[n / 10] + FACTORIALS[n % 10];
    }

    // pre-fill known cycle members
    loop_length_map[1] = 1;
    loop_length_map[2] = 1;
    loop_length_map[145] = 1;
    loop_length_map[40585] = 1;
    loop_length_map[169] = 3;
    loop_length_map[363601] = 3;
    loop_length_map[1454] = 3;
    loop_length_map[871] = 2;
    loop_length_map[45361] = 2;
    loop_length_map[872] = 2;
    loop_length_map[45362] = 2;

    for n in 1u32..LIMIT {
        chain_list.clear();

        let mut next = n;

        loop {
            if loop_length_map[next as usize] != 0 {
                let total_len = chain_list.len() as u8 + loop_length_map[next as usize];
                for (i, &x) in chain_list.iter().enumerate() {
                    loop_length_map[x as usize] = total_len - i as u8;
                }
                if total_len == 60 {
                    chain_count += 1;
                }
                break;
            }

            chain_list.push(next);
            next = next_cache[next as usize];
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
