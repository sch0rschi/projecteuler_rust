pub fn solve_0056() -> u32 {
    let mut max_sum = 0;

    for a in 1u32..100 {
        let mut digits = vec![1u8];
        let mut best_for_a = 0;

        for _ in 1..100 {
            multiply_in_place(&mut digits, a);

            let sum: u32 = digits.iter().map(|&d| d as u32).sum();
            if sum > best_for_a {
                best_for_a = sum;
            }
        }

        if best_for_a > max_sum {
            max_sum = best_for_a;
        }
    }

    max_sum
}

fn multiply_in_place(digits: &mut Vec<u8>, mul: u32) {
    let mut carry = 0u32;

    for d in digits.iter_mut() {
        let prod = (*d as u32) * mul + carry;
        *d = (prod % 10) as u8;
        carry = prod / 10;
    }

    while carry > 0 {
        digits.push((carry % 10) as u8);
        carry /= 10;
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0056::solve_0056;

    #[test]
    fn test() {
        solve_print_and_check(solve_0056, 972);
    }
}
