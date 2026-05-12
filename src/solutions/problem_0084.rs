use nalgebra::{DMatrix, DVector};

const SQUARES: usize = 40;
const JAIL: usize = 10;
const G2J: usize = 30;
const DICE_SIDES: usize = 4;

// Exact stationary distribution via Markov chain + linear system solve (T-I)v=0.
// Avoids the non-determinism and slow convergence of the Monte Carlo approach.
pub fn solve_0084() -> usize {
    const STATES: usize = SQUARES * 3;
    let mut trans = DMatrix::<f64>::zeros(STATES, STATES);

    for sq in 0..SQUARES {
        for dc in 0..3usize {
            let from = sq * 3 + dc;
            for d1 in 1..=DICE_SIDES {
                for d2 in 1..=DICE_SIDES {
                    let prob = 1.0 / (DICE_SIDES * DICE_SIDES) as f64;
                    let is_double = d1 == d2;
                    let new_dc = if is_double { dc + 1 } else { 0 };

                    if is_double && new_dc == 3 {
                        trans[(JAIL * 3, from)] += prob;
                        continue;
                    }

                    let moved = (sq + d1 + d2) % SQUARES;
                    for (dest, p) in resolve(moved) {
                        let to = dest * 3 + new_dc;
                        trans[(to, from)] += prob * p;
                    }
                }
            }
        }
    }

    for from in 0..STATES {
        let col_sum: f64 = (0..STATES).map(|to| trans[(to, from)]).sum();
        if (col_sum - 1.0).abs() > 1e-9 {
            println!("col {from} (sq={} dc={}) sums to {col_sum}", from / 3, from % 3);
        }
    }

    let mut a = trans - DMatrix::<f64>::identity(STATES, STATES);
    for j in 0..STATES {
        a[(STATES - 1, j)] = 1.0;
    }

    let mut b = DVector::<f64>::zeros(STATES);
    b[STATES - 1] = 1.0;

    let v = a.lu().solve(&b).expect("singular");

    let mut sq_prob: Vec<(usize, f64)> = (0..SQUARES)
        .map(|sq| (sq, v[sq*3] + v[sq*3+1] + v[sq*3+2]))
        .collect();
    sq_prob.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    sq_prob[0].0 * 10000 + sq_prob[1].0 * 100 + sq_prob[2].0
}

fn resolve(sq: usize) -> Vec<(usize, f64)> {
    match sq {
        G2J => vec![(JAIL, 1.0)],
        2 | 17 | 33 => vec![
            (0,    1.0/16.0),
            (JAIL, 1.0/16.0),
            (sq,   14.0/16.0),
        ],
        7 | 22 | 36 => {
            let back3 = sq - 3;
            let mut v = vec![
                (0,          1.0/16.0),
                (JAIL,       1.0/16.0),
                (11,         1.0/16.0),
                (24,         1.0/16.0),
                (39,         1.0/16.0),
                (5,          1.0/16.0),
                (next_r(sq), 2.0/16.0),
                (next_u(sq), 1.0/16.0),
                (sq,         6.0/16.0), // stay: 6 cards do nothing
            ];
            for (dest, p) in resolve(back3) {
                v.push((dest, p / 16.0));
            }
            v
        }
        _ => vec![(sq, 1.0)],
    }
}

fn next_r(x: usize) -> usize {
    (((x + 5) / 10) * 10 + 5) % SQUARES
}

fn next_u(x: usize) -> usize {
    if !(12..28).contains(&x) { 12 } else { 28 }
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0084::solve_0084;

    #[test]
    fn test() {
        solve_print_and_check(solve_0084, 101524);
    }
}
