use crate::libs::digits::reverse;

pub fn solve_0055() -> u32 {
    (1u64..10_000).filter(|&i| is_lychrel(i)).count() as u32
}

fn is_lychrel(mut n: u64) -> bool {
    for _ in 0..50 {
        let rev = reverse(n);
        n += rev;
        if n == reverse(n) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0055::solve_0055;

    #[test]
    fn test() {
        solve_print_and_check(solve_0055, 249);
    }
}
