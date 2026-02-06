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
    let calweek = format!("{}{:02}", year_str, week_number);

    CalWeekResult {
        calweek,
        year: year_str,
        week_number,
    }
}

pub fn get_current_calweek() -> CalWeekResult {
    get_calweek_by_date(Utc::now().date_naive())
}

pub fn get_monday_and_sunday(calweek: &str) -> WeekRangeResult {
    let year: i32 = calweek[..2].parse::<i32>().unwrap() + 2000;
    let week_number: u32 = calweek[calweek.len() - 2..].parse::<u32>().unwrap();

    let monday_date: NaiveDate = NaiveDate::from_isoywd_opt(year, week_number, Weekday::Mon).unwrap();
    let sunday_date: NaiveDate = NaiveDate::from_isoywd_opt(year, week_number, Weekday::Sun).unwrap();

    let str_monday: String = monday_date.format("%d.%m.%Y").to_string();
    let str_sunday: String = sunday_date.format("%d.%m.%Y").to_string();

    WeekRangeResult {
        monday: str_monday,
        sunday: str_sunday,
    }
}
