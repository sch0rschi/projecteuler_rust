pub fn solve_0055() -> i32 {
    (1u64..10000).filter(|&i| is_lychrel(i)).count() as i32
}

fn is_lychrel(n: u64) -> bool {
    let mut n = n as u128;
    for _ in 0..50 {
        n += reverse(n);
        if is_palindrome(n) { return false; }
    }
    true
}

#[inline(always)]
fn reverse(mut n: u128) -> u128 {
    let mut r = 0u128;
    while n > 0 {
        r = r * 10 + n % 10;
        n /= 10;
    }
    r
}

#[inline(always)]
fn is_palindrome(n: u128) -> bool {
    let mut digits = [0u8; 40];
    let mut len = 0usize;
    let mut m = n;
    while m > 0 {
        digits[len] = (m % 10) as u8;
        len += 1;
        m /= 10;
    }
    let mut lo = 0;
    let mut hi = len - 1;
    while lo < hi {
        if digits[lo] != digits[hi] { return false; }
        lo += 1;
        hi -= 1;
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
