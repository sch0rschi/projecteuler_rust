const LIMIT: usize = 12_000;
const LIMIT2: u32 = 2 * LIMIT as u32;

pub fn solve_0088() -> u32 {
    let mut min_product_sum = vec![LIMIT2; LIMIT + 1];

    // Stack holds (product, sum, terms, min_factor)
    let mut stack: Vec<(u32, u32, u32, u32)> = Vec::with_capacity(64);

    for i in 2u32..=LIMIT as u32 {
        stack.push((i, i, 1, i));
        while let Some((product, sum, terms, min_f)) = stack.pop() {
            let k = (terms + product - sum) as usize;
            if k <= LIMIT {
                if product < min_product_sum[k] {
                    min_product_sum[k] = product;
                }
                let max_f = LIMIT2 / product;
                for f in min_f..=max_f {
                    stack.push((product * f, sum + f, terms + 1, f));
                }
            }
        }
    }

    let mut seen = vec![false; LIMIT2 as usize + 1];
    let mut total = 0u32;
    for &v in &min_product_sum[2..=LIMIT] {
        if !seen[v as usize] {
            seen[v as usize] = true;
            total += v;
        }
    }
    total
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
