use projecteuler::integer_pythagorean_triplets::{expand, R};
use projecteuler::triplet::Triplet;

fn main() {
    if let Some(solution_triplet) = expansion_recursion(R) {
        let scaling = 1000 / solution_triplet.sum();
        print!("{}", solution_triplet.product() * scaling * scaling * scaling);
    }
}

fn expansion_recursion(triplet: Triplet) -> Option<Triplet> {
    match triplet.sum() {
        sum  if 1000 % sum == 0 => Some(triplet),
        (1001..) => None,
        _ => {
            let (expansion_1, expansion_2, expansion_3) = expand(triplet);
            if let Some(expansion) = expansion_recursion(expansion_1) {
                return Some(expansion);
            }
            if let Some(expansion) = expansion_recursion(expansion_2) {
                return Some(expansion);
            }
            if let Some(expansion) = expansion_recursion(expansion_3) {
                return Some(expansion);
            }
            None
        }
    }
}
