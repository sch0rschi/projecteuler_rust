use crate::libs::integer_pythagorean_triplets::{expand, R};

const LIMIT: u64 = 1_000_000_000;


pub fn solve_0094() -> u64 {
    let mut sum: u64 = 0;

    let mut search_space_stack = vec![R];

    while let Some(triplet) = search_space_stack.pop() {
        let hypotenuse = triplet.2;
        if hypotenuse as u64 > LIMIT / 3 + 1 {
            continue;
        }

        let shorter = triplet.0.min(triplet.1);

        if 2 * shorter == hypotenuse + 1 {
            sum += 3 * hypotenuse as u64 - 1;
        }
        if 2 * shorter == hypotenuse - 1 {
            sum += 3 * hypotenuse as u64 + 1;
        }

        let (a, b, c) = expand(triplet);
        search_space_stack.push(a);
        search_space_stack.push(b);
        search_space_stack.push(c);
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
