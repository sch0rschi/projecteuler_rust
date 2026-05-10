pub fn solve_0045() -> i32 {
    // all hexagonal numbers are also triangle numbers, hence ignoring triangle numbers

    // Starting after known solution 40755:
    // P(165) = H(143) = 40755

    let mut p_j = 40755;
    let mut j = 165;

    let mut h_k = 40755;
    let mut k = 143;

    loop {
        if p_j <= h_k {
            p_j += 3 * j + 1;
            j += 1;
        } else {
            h_k += 4 * k + 1;
            k += 1;
        }

        if p_j == h_k {
            return p_j;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0045::solve_0045;

    #[test]
    fn test() {
        solve_print_and_check(solve_0045, 1533776805);
    }
}
