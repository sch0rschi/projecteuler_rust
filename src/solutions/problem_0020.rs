pub fn solve_0020() -> u32 {
    let mut digits = Vec::with_capacity(200);
    digits.push(1u16);

    for n in 2..=100 {
        let mut carry = 0;

        for d in digits.iter_mut() {
            let val = *d * n + carry;
            *d = val % 10;
            carry = val / 10;
        }

        while carry > 0 {
            digits.push(carry % 10);
            carry /= 10;
        }
    }

    digits.iter().map(|&d| d as u32).sum()
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0020::solve_0020;

    #[test]
    fn test() {
        solve_print_and_check(solve_0020, 648)
    }
}
