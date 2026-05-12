pub fn solve_0045() -> i64 {
    // all hexagonal numbers are also triangle numbers, hence ignoring triangle numbers

    // Starting after known solution 40755:
    // P(165) = H(143) = 40755

    let mut hi = 144i64;
    let mut pi = 165i64;
    let mut h = hi * (2 * hi - 1);
    let mut p = pi * (3 * pi - 1) / 2;

    loop {
        match p.cmp(&h) {
            std::cmp::Ordering::Less => {
                pi += 1;
                p = pi * (3 * pi - 1) / 2;
            }
            std::cmp::Ordering::Greater => {
                hi += 1;
                h = hi * (2 * hi - 1);
            }
            std::cmp::Ordering::Equal => return p,
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
