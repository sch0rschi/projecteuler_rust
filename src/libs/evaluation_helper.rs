use std::fmt::{Debug, Display};
use std::time::Instant;

pub fn solve_print_and_check<T>(solver: fn() -> T, expected: T)
where
    T: Display + PartialEq + Debug,
{
    let start = Instant::now();
    let result = solver();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(expected, result);
    assert!(duration < std::time::Duration::from_secs(1));
}
