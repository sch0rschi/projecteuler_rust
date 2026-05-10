pub fn solve_0055() -> i32 {
    let mut count = 0;

    for i in 1u128..10000 {
        if is_lychrel(i) {
            count += 1;
        }
    }

    count
}

fn is_lychrel(mut n: u128) -> bool {
    for _ in 1..=50 {
        n += reverse(n);
        if n == reverse(n) {
            return false;
        }
    }
    true
}

fn reverse(mut n: u128) -> u128 {
    let mut result = 0u128;
    while n > 0 {
        result = result * 10 + n % 10;
        n /= 10;
    }
    result
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
