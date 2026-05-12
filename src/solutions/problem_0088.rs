use itertools::Itertools;

const LIMIT: usize = 12_000;


pub fn solve_0088() -> u32 {
    let mut min_product_sum = vec![u32::MAX; LIMIT + 1];

    for i in 2u32..LIMIT as u32 + 1 {
        search(i, i, 1, i, &mut min_product_sum);
    }

    min_product_sum[2..=LIMIT].iter().unique().sum()
}

fn search(
    product: u32,
    sum: u32,
    terms: u32,
    min_factor: u32,
    min_product_sum: &mut Vec<u32>,
) {
    let k = (terms + product - sum) as usize;
    if k > LIMIT {
        return;
    }
    min_product_sum[k] = min_product_sum[k].min(product);

    for f in min_factor..=LIMIT as u32 {
        let new_product = product * f;
        if new_product > 2 * LIMIT as u32 {
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
