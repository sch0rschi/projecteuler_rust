use crate::libs::integer_pythagorean_triplets::{expand_1_3, R};
use crate::libs::triplet::Triplet;

const LIMIT: i64 = 1_000_000_000;

pub fn solve_0094() -> i64 {
    let mut sum: i64 = 0;

    let mut search_space_stack = vec![R];

    while let Some(triplet) = search_space_stack.pop() {
        let Triplet(a, b, c) = triplet;
        let (a, b, c) = (a as i64, b as i64, c as i64);

        if c > LIMIT / 3 + 1 {
            continue;
        }

        let shorter = a.min(b);
        if 2 * shorter == c + 1 {
            sum += 3 * c - 1;
        }
        if 2 * shorter == c - 1 {
            sum += 3 * c + 1;
        }

        let Triplet(a2, b2, c2) = triplet;
        let (t1, t3) = expand_1_3(Triplet(a2, b2, c2));
        search_space_stack.push(t1);
        // we don't need this expansion,
        // because this leg produces triplets where a and b move apart from each other
        // search_space_stack.push(t2);
        search_space_stack.push(t3);
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0094::solve_0094;

    #[test]
    fn test() {
        solve_print_and_check(solve_0094, 518408346);
    }
}
