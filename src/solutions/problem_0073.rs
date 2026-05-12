const LIMIT: u32 = 12_000;

// https://en.wikipedia.org/wiki/Farey_sequence
pub fn solve_0073() -> u32 {
    count(0, 1, 1, 1)
}

fn count(a: u32, b: u32, c: u32, d: u32) -> u32 {
    let mediant_num = a + c;
    let mediant_den = b + d;

    if mediant_den > LIMIT {
        return 0;
    }

    if mediant_num * 3 <= mediant_den {
        return count(mediant_num, mediant_den, c, d);
    }

    if mediant_num * 2 >= mediant_den {
        return count(a, b, mediant_num, mediant_den);
    }

    1 + count(a, b, mediant_num, mediant_den) + count(mediant_num, mediant_den, c, d)
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0073::solve_0073;

    #[test]
    fn test() {
        solve_print_and_check(solve_0073, 7295372);
    }
}
