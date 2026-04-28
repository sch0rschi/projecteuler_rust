use bitvec::bitvec;
use projecteuler::evaluation_helper::solve_print_and_check;
use std::fs;

fn main() {
    solve_print_and_check(solve_0079, 73162890);
}

fn solve_0079() -> u64 {
    let input = fs::read_to_string("resources/0079_keylog.txt").expect("Failed to read file");

    let mut adj: [Vec<u8>; 10] = std::array::from_fn(|_| Vec::new());
    let mut indegree = [0i8; 10];
    let mut to_be_used = bitvec![0; 10];

    for line in input.lines().filter(|l| !l.is_empty()) {
        let digits = line.as_bytes();
        let digit_1 = (digits[0] - b'0') as usize;
        let digit_2 = (digits[1] - b'0') as usize;
        let digit_3 = (digits[2] - b'0') as usize;

        to_be_used.set(digit_1, true);
        to_be_used.set(digit_2, true);
        to_be_used.set(digit_3, true);

        if !adj[digit_1].contains(&(digit_2 as u8)) {
            adj[digit_1].push(digit_2 as u8);
            indegree[digit_2] += 1;
        }

        if !adj[digit_2].contains(&(digit_3 as u8)) {
            adj[digit_2].push(digit_3 as u8);
            indegree[digit_3] += 1;
        }
    }

    for (value, is_used) in to_be_used.iter().enumerate() {
        if !*is_used {
            indegree[value] = -1;
        }
    }

    let total = to_be_used.count_ones();
    let mut result = 0u64;

    for _ in 0..total {
        let n = get_next_with_resolved_dependencies(&indegree);

        result = result * 10 + n as u64;

        for &to in &adj[n] {
            indegree[to as usize] -= 1;
        }
        indegree[n] -= 1;
    }

    result
}

#[inline(always)]
fn get_next_with_resolved_dependencies(indegrees: &[i8; 10]) -> usize {
    for (value, &in_degree) in indegrees.iter().enumerate() {
        if in_degree == 0 {
            return value;
        }
    }
    unreachable!("No next node with resolved dependencies found");
}
