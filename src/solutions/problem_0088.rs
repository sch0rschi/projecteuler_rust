use itertools::Itertools;

const LIMIT: usize = 12_000;


pub fn solve_0088() -> u64 {
    let mut min_product_sum = vec![u64::MAX; LIMIT + 1];

    search(1, 1, 1, 2, &mut min_product_sum);
    min_product_sum[2..=LIMIT].iter().unique().sum()
}

fn search(
    product: usize,
    sum: usize,
    terms: usize,
    min_factor: usize,
    min_product_sum: &mut Vec<u64>,
) {
    let k = terms + product - sum;
    if k > LIMIT {
        return;
    }

    if terms >= 1 {
        if (product as u64) < min_product_sum[k] {
            min_product_sum[k] = product as u64;
        }
    }

    for f in min_factor..=LIMIT {
        let new_product = product * f;
        if new_product > 2 * LIMIT {
            break;
        }
        search(new_product, sum + f, terms + 1, f, min_product_sum);
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0088::solve_0088;

    #[test]
    fn test() {
        solve_print_and_check(solve_0088, 7587457);
    }
}
