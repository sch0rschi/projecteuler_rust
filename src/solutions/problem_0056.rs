pub fn solve_0056() -> u32 {
    let mut max_sum = 0u32;

    for a in 2u32..100 {
        let mut digits = [0u8; 200];
        let mut len = 1usize;
        digits[0] = 1;

        for _ in 1..100 {
            let sum = multiply_and_sum(&mut digits, &mut len, a);
            if sum > max_sum {
                max_sum = sum;
            }
        }
    }

    max_sum
}

#[inline(always)]
fn multiply_and_sum(digits: &mut [u8; 200], len: &mut usize, mul: u32) -> u32 {
    let mut carry = 0u32;
    let mut sum = 0u32;

    for d in digits[..*len].iter_mut() {
        let prod = (*d as u32) * mul + carry;
        *d = (prod % 10) as u8;
        carry = prod / 10;
        sum += *d as u32;
    }

    while carry > 0 {
        digits[*len] = (carry % 10) as u8;
        sum += digits[*len] as u32;
        *len += 1;
        carry /= 10;
    }

    sum
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
