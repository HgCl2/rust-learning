use chrono::Datelike;
pub use chrono::{NaiveDate, Utc, TimeZone};
pub use chrono::Weekday as wd;

pub fn middle_day(year: usize) -> Option<wd> {
    let middle_day_odd_year = 365 / 2 + 1;
    if year % 4 != 0{
        let date = NaiveDate::from_yo_opt(year as i32, middle_day_odd_year).unwrap();
        Some(date.weekday())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(middle_day(1022).unwrap(), wd::Tue);
    }
}
