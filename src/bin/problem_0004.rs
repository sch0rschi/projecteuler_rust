use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0004();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(906609, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0004() -> i32 {
    let mut max_palindrome = 0;
    for i_1 in 100..=999 {
        for i_2 in 100..=999 {
            if is_palindrome_number(i_1 * i_2) && i_1 * i_2 > max_palindrome {
                max_palindrome = i_1 * i_2;
            }
        }
    }
    max_palindrome
}

fn is_palindrome_number(n: i32) -> bool {
    let n_string = n.to_string();
    let n_string_reversed = n_string.chars().rev().collect::<String>();
    n_string == n_string_reversed
}