use crate::libs::pentagonal_numbers::{is_pentagonal, pentagonal};

pub fn solve_0044() -> usize {
    let mut pent = Vec::new();
    let mut best = usize::MAX;

    let mut j = 1usize;
    loop {
        let pj = pentagonal(j);

        if pj.saturating_sub(1) >= best {
            break;
        }

        for i in (0..pent.len()).rev() {
            let pi = pent[i];
            let diff = pj - pi;

            if diff >= best {
                break;
            }

            if is_pentagonal(diff) && is_pentagonal(pj + pi) {
                best = diff;
            }
        }

        pent.push(pj);
        j += 1;
    }

    best
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
