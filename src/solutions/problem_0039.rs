use crate::libs::integer_pythagorean_triplets::{expand, R};
use crate::libs::triplet::Triplet;


pub fn solve_0039() -> usize {
    let mut solutions_per_perimeter = [0i32; 1001];
    count_perimeters(R, &mut solutions_per_perimeter);

    for perimeter in (1..solutions_per_perimeter.len()).rev() {
        let count = solutions_per_perimeter[perimeter];
        let max_multiplier = 1000 / perimeter;
        for multiplier in 2..=max_multiplier {
            solutions_per_perimeter[perimeter * multiplier] += count;
        }
    }

    solutions_per_perimeter
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .unwrap()
        .0
}

fn count_perimeters(root_triplet: Triplet, solutions_per_perimeter: &mut [i32; 1001]) {
    let mut stack = [Triplet(0, 0, 0); 25];
    let mut top = 0usize;
    stack[top] = root_triplet;
    top += 1;

    while top > 0 {
        top -= 1;
        let triplet = stack[top];
        let sum = triplet.sum();
        if sum <= 1000 {
            solutions_per_perimeter[sum] += 1;

            let (exp1, exp2, exp3) = expand(triplet);
            stack[top] = exp1;
            stack[top + 1] = exp2;
            stack[top + 2] = exp3;
            top += 3;
        }
    }
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
