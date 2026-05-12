// https://en.wikipedia.org/wiki/Farey_sequence
pub fn solve_0071() -> i32 {
    let limit = 1_000_000;
    let k = (limit + 1) / 7;
    k * 3 - 1
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0071::solve_0071;

    #[test]
    fn test() {
        solve_print_and_check(solve_0071, 428570);
    }
}
