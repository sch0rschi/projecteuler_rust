const NUMBER_OF_RINGS: u32 = 500;

// Four corners of ring k, summed over k=1..=n:
// top-right:    Σ(2k+1)^2      = 4*Σk^2 + 4*Σk + n
// top-left:     Σ(2k+1)^2 - 2k = 4*Σk^2 + 2*Σk + n
// bottom-left:  Σ(2k+1)^2 - 4k = 4*Σk^2        + n
// bottom-right: Σ(2k+1)^2 - 6k = 4*Σk^2 - 2*Σk + n
pub fn solve_0028() -> u32 {
    let sum_k = NUMBER_OF_RINGS * (NUMBER_OF_RINGS + 1) / 2;
    let sum_k_squared = NUMBER_OF_RINGS * (NUMBER_OF_RINGS + 1) * (2 * NUMBER_OF_RINGS + 1) / 6;

    let top_right = 4 * sum_k_squared + 4 * sum_k + NUMBER_OF_RINGS;
    let top_left = 4 * sum_k_squared + 2 * sum_k + NUMBER_OF_RINGS;
    let bottom_left = 4 * sum_k_squared + NUMBER_OF_RINGS;
    let bottom_right = 4 * sum_k_squared - 2 * sum_k + NUMBER_OF_RINGS;

    1 + top_right + top_left + bottom_left + bottom_right
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0028::solve_0028;

    #[test]
    fn test() {
        solve_print_and_check(solve_0028, 669171001);
    }
}
