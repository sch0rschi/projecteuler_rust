use chrono::{Datelike, NaiveDate, Weekday};

pub fn solve_0019() -> i32 {
    let mut count = 0;

    for year in 1901..=2000 {
        for month in 1..=12 {
            let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

            if date.weekday() == Weekday::Sun {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0019::solve_0019;

    #[test]
    fn test() {
        solve_print_and_check(solve_0019, 171);
    }
}
