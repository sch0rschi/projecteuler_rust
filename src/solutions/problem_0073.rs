const LIMIT: u32 = 12_000;

// https://en.wikipedia.org/wiki/Farey_sequence
pub fn solve_0073() -> i32 {

    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    let mut d = LIMIT;

    let mut count = 0;

    while !(c == 1 && d == 2) {
        let k = (LIMIT + b) / d;

        let e = k * c - a;
        let f = k * d - b;

        a = c;
        b = d;
        c = e;
        d = f;

        if 3 * a > b && 2 * a < b {
            count += 1;
        }
    }

    count
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
