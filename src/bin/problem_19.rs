use chrono::{Datelike, NaiveDate, Weekday};

fn main() {
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
    println!("{}", counter);
 }