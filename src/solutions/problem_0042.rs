use crate::libs::word_score::score;
use std::fs;


pub fn solve_0042() -> usize {
    fs::read("resources/0042_words.txt")
        .unwrap()
        .split(|&b| b == b'"')
        .filter(|&word| !word.is_empty() && word != b"," && is_triangle_number(score(word)))
        .count()
}

fn is_triangle_number(x: u32) -> bool {
    let discriminant = 1 + 8 * x as u64;
    let sqrt = (discriminant as f64).sqrt() as u64;
    sqrt * sqrt == discriminant && (sqrt - 1).is_multiple_of(2)
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0042::solve_0042;

    #[test]
    fn test() {
        solve_print_and_check(solve_0042, 162);
    }
}
