use chrono::{Datelike, NaiveDate, Weekday};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0019();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(171, result);
    assert!(duration < std::time::Duration::from_secs(1));
}
fn solve_0019() -> i32 {
    let start = NaiveDate::from_ymd_opt(1901, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2000, 12, 31).unwrap();

    let mut counter = 0;
    let mut date = start;
    while date <= end {
        if date.day() == 1 && date.weekday() == Weekday::Sun {
            counter += 1;
        }
        date = date.succ_opt().unwrap()
    }
    counter
}