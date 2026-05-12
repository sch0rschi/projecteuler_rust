pub fn solve_0044() -> usize {
    const N: usize = 3000;

    let pent: Vec<usize> = (1..N).map(pentagonal).collect();

    let max_val = pent[N - 2] + pent[N - 2];

    let mut is_pent = vec![false; max_val + 1];
    for &p in &pent {
        is_pent[p] = true;
    }

    let mut best = usize::MAX;

    for j in 1..pent.len() {
        let pj = pent[j];

        for i in 0..j {
            let pi = pent[i];
            let diff = pj - pi;

            if diff >= best {
                break;
            }

            if is_pent[diff] && is_pent[pj + pi] {
                best = diff;
            }
        }
    }

    best
}

fn pentagonal(n: usize) -> usize {
    n * (3 * n - 1) / 2
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0044::solve_0044;

    #[test]
    fn test() {
        solve_print_and_check(solve_0044, 5482660);
    }
}
