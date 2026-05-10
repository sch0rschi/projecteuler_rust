use num_integer::Integer;


pub fn solve_0036() -> u64 {
    let mut sum = 0;
    for i in 1..999 {
        let p1 = make_odd_palindrome(i);
        let p2 = make_even_palindrome(i);
        if is_binary_palindrome(p1) {
            sum += p1;
        }
        if is_binary_palindrome(p2) {
            sum += p2;
        }
    }
    sum
}

fn make_even_palindrome(mut n: u64) -> u64 {
    let mut result = n;
    let mut rem;
    while n > 0 {
        (n, rem) = n.div_rem(&10);
        result = result * 10 + rem;
    }
    result
}

fn make_odd_palindrome(mut n: u64) -> u64 {
    let mut result = n;
    n /= 10;
    let mut rem;
    while n > 0 {
        (n, rem) = n.div_rem(&10);
        result = result * 10 + rem;
    }
    result
}

fn is_binary_palindrome(n: u64) -> bool {
    if n.is_multiple_of(2) {
        return false;
    }
    let reversed = n.reverse_bits();
    let leading_zeros = n.leading_zeros();
    reversed >> leading_zeros == n
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0036::solve_0036;

    #[test]
    fn test() {
        solve_print_and_check(solve_0036, 872187);
    }
}
