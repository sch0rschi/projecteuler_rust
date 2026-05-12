use crate::libs::integer_pythagorean_triplets::{expand, R};
use crate::libs::triplet::Triplet;

pub fn solve_0039() -> usize {
    let mut counts = [0u8; 1001];

    let mut stack = [Triplet(0, 0, 0); 32];
    let mut top = 0;

    stack[top] = R;
    top += 1;

    while top > 0 {
        top -= 1;

        let triplet = stack[top];
        let p = triplet.sum();

        if p > 1000 {
            continue;
        }

        // propagate multiples immediately
        let mut k = p;
        while k <= 1000 {
            counts[k] += 1;
            k += p;
        }

        let (a, b, c) = expand(triplet);

        stack[top] = a;
        stack[top + 1] = b;
        stack[top + 2] = c;
        top += 3;
    }

    counts
        .iter()
        .enumerate()
        .max_by_key(|&(_, c)| c)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0039::solve_0039;

    #[test]
    fn test() {
        solve_print_and_check(solve_0039, 840);
    }
}
