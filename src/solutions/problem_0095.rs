use crate::libs::divisors::ProperDivisorSums;

const LIMIT: usize = 1_000_000;

pub fn solve_0095() -> u32 {
    let proper_divisor_sums = ProperDivisorSums::new(LIMIT);

    // None = unvisited
    // Some(true) = visiting
    // Some(false) = processed
    let mut state: Vec<Option<bool>> = vec![None; LIMIT + 1];

    let mut best_len = 0;
    let mut best_min = 0;

    let mut path = Vec::with_capacity(1024);

    for start in 1..=LIMIT {
        if state[start].is_some() {
            continue;
        }

        let mut cur = start as u32;
        path.clear();

        while (cur as usize) <= LIMIT {
            match state[cur as usize] {
                None => {
                    state[cur as usize] = Some(true);
                    path.push(cur);
                    cur = proper_divisor_sums.get(cur);
                }
                Some(true) => {
                    let cycle_start = cur;

                    let mut idx = path.len();
                    while idx > 0 {
                        idx -= 1;
                        if path[idx] == cycle_start {
                            break;
                        }
                    }

                    let cycle = &path[idx..];

                    let len = cycle.len();
                    let min_val = *cycle.iter().min().unwrap();

                    if len > best_len {
                        best_len = len;
                        best_min = min_val;
                    }

                    for &v in cycle {
                        state[v as usize] = Some(false);
                    }

                    break;
                }
                Some(false) => {
                    break;
                }
            }
        }

        for &v in &path {
            state[v as usize] = Some(false);
        }
    }

    best_min
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0095::solve_0095;

    #[test]
    fn test() {
        solve_print_and_check(solve_0095, 14316);
    }
}
