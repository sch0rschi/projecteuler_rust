use std::ops::Mul;
use nalgebra::{Matrix3, Vector3};

static A: Matrix3<i32> = Matrix3::new(1, -2, 2, 2, -1, 2, 2, -2, 3);

static B: Matrix3<i32> = Matrix3::new(1, 2, 2, 2, 1, 2, 2, 2, 3);

static C: Matrix3<i32> = Matrix3::new(-1, 2, 2, -2, 1, 2, -2, 2, 3);

fn main() {
    let r = Vector3::new(3, 4, 5);
    if let Some(solution_triplet) = expansion_recursion(&r) {
        let scaling = 1000 / solution_triplet.sum();
        let solution_triplet = solution_triplet.mul(scaling);
        print!("{}", solution_triplet.product());
    }
}

fn expansion_recursion(triplet: &Vector3<i32>) -> Option<Vector3<i32>> {
    match triplet.sum() {
        sum  if 1000 % sum == 0 => Some(*triplet),
        (1001..) => None,
        _ => {
            let (expansion_1, expansion_2, expansion_3) = expand(triplet);
            if let Some(expansion) = expansion_recursion(&expansion_1) {
                return Some(expansion);
            }
            if let Some(expansion) = expansion_recursion(&expansion_2) {
                return Some(expansion);
            }
            if let Some(expansion) = expansion_recursion(&expansion_3) {
                return Some(expansion);
            }
            None
        }
    }
}

fn expand(triplet: &Vector3<i32>) -> (Vector3<i32>, Vector3<i32>, Vector3<i32>) {
    (A * triplet, B * triplet, C * triplet)
}
