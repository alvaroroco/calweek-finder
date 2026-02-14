use chrono::{Datelike, NaiveDate, Utc, Weekday};
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct CalWeekResult {
    pub calweek: String,
    pub year: String,
    pub week_number: u32,
}

impl fmt::Display for CalWeekResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CalWeek: {}", self.calweek)
    }
}

#[derive(Serialize, Debug)]
pub struct WeekRangeResult {
    pub monday: String,
    pub sunday: String,
}

impl fmt::Display for WeekRangeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monday: {}\nSunday: {}", self.monday, self.sunday)
    }
}

fn get_week_number(date_obj: NaiveDate) -> u32 {
    date_obj.iso_week().week()
}

pub fn get_calweek_by_date(date: NaiveDate) -> CalWeekResult {
    let year_str = format!("{:02}", date.year() % 100);
    let week_number = get_week_number(date);
    let calweek = format!("{year_str}{week_number:02}");

    CalWeekResult {
        calweek,
        year: year_str,
        week_number,
    }
}

pub fn get_current_calweek() -> CalWeekResult {
    get_calweek_by_date(Utc::now().date_naive())
}

pub fn get_monday_and_sunday(calweek: &str) -> Result<WeekRangeResult, String> {
    if calweek.len() != 4 || !calweek.chars().all(|c| c.is_ascii_digit()) {
        return Err(String::from("CalWeek must be a 4-digit value (e.g., 2348)"));
    }

    let year = calweek[..2]
        .parse::<i32>()
        .map_err(|_| String::from("CalWeek year is not valid"))?
        + 2000;
    let week_number = calweek[2..]
        .parse::<u32>()
        .map_err(|_| String::from("CalWeek week number is not valid"))?;

    let monday_date = NaiveDate::from_isoywd_opt(year, week_number, Weekday::Mon)
        .ok_or_else(|| String::from("CalWeek week number is out of ISO range"))?;
    let sunday_date = NaiveDate::from_isoywd_opt(year, week_number, Weekday::Sun)
        .ok_or_else(|| String::from("CalWeek week number is out of ISO range"))?;
    let str_monday = monday_date.format("%d.%m.%Y").to_string();
    let str_sunday = sunday_date.format("%d.%m.%Y").to_string();

    Ok(WeekRangeResult {
        monday: str_monday,
        sunday: str_sunday,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn converts_date_to_calweek() {
        let date = NaiveDate::from_ymd_opt(2023, 11, 26).expect("valid date");
        let result = get_calweek_by_date(date);

        assert_eq!(result.calweek, "2347");
        assert_eq!(result.week_number, 47);
        assert_eq!(result.year, "23");
    }

    #[test]
    fn rejects_non_numeric_calweek() {
        let result = get_monday_and_sunday("abcd");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_out_of_range_iso_week() {
        let result = get_monday_and_sunday("2454");
        assert!(result.is_err());
    }

    #[test]
    fn converts_valid_calweek_to_date_range() {
        let result = get_monday_and_sunday("2348").expect("valid calweek");
        assert_eq!(result.monday, "27.11.2023");
        assert_eq!(result.sunday, "03.12.2023");
    }
}
