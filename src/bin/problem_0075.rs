use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::integer_pythagorean_triplets::{expand, R};

const LIMIT: usize = 1_500_000;
fn main() {
    solve_print_and_check(solve_0075, 161667);
}

fn solve_0075() -> usize {
    let mut counters = vec![0u32; LIMIT + 1];

    let mut search_space_stack = vec![R];

    while let Some(triplet) = search_space_stack.pop() {
        let diameter = triplet.sum();

        if diameter > LIMIT {
            continue;
        }

        let mut k = diameter;
        while k <= LIMIT {
            counters[k] += 1;
            k += diameter;
        }

        let (a, b, c) = expand(triplet);
        search_space_stack.push(a);
        search_space_stack.push(b);
        search_space_stack.push(c);
    }

    counters.iter().filter(|&&x| x == 1).count()
}
