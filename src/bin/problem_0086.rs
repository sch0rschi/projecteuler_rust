use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::integer_pythagorean_triplets::{expand, R};

fn main() {
    solve_print_and_check(solve_0086, 1818);
}

fn solve_0086() -> i32 {
    let target = 1_000_000;

    let mut upper = 1;
    while count_integer_shortest_paths_for_m(upper) <= target {
        upper *= 2;
    }
    let mut lower = upper / 2 + 1;

    while lower < upper {
        let mid = (lower + upper) / 2;
        if count_integer_shortest_paths_for_m(mid) <= target {
            lower = mid + 1;
        } else {
            upper = mid;
        }
    }

    upper
}

fn count_integer_shortest_paths_for_m(m: i32) -> i32 {
    let mut counter = 0;

    let mut search_space_stack = vec![R];

    // always assuming l_1 >= l_2 >= l_3
    while let Some(triplet) = search_space_stack.pop() {
        if triplet.sum() as i32 > 6 * m {
            continue;
        }

        let longer = triplet.0.max(triplet.1);
        let shorter = triplet.0.min(triplet.1);

        // l_1 longest leg anyways
        for scale in 1..=(m / longer) {
            counter += shorter * scale / 2;
        }
        // l_1 longest leg, not always
        for scale in 1..=(m / shorter) {
            let max_l2 = scale * longer / 2;
            let min_l2 = (scale * (longer - shorter)).max(1);
            if max_l2 >= min_l2 {
                counter += max_l2 - min_l2 + 1;
            }
        }

        let (a, b, c) = expand(triplet);
        search_space_stack.push(a);
        search_space_stack.push(b);
        search_space_stack.push(c);
    }

    counter
}
