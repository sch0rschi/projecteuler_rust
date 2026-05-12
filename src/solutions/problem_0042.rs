use crate::libs::word_score::score;

const WORDS: &str = include_str!("../../resources/0042_words.txt");

pub fn solve_0042() -> usize {
    let mut tri = [false; 1000];

    let mut n = 1;
    let mut t = 1;
    while t < tri.len() {
        tri[t] = true;
        n += 1;
        t = n * (n + 1) / 2;
    }

    WORDS
        .split('"')
        .filter(|&w| !w.is_empty() && w != ",")
        .filter(|&w| tri[score(w) as usize])
        .count()
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
