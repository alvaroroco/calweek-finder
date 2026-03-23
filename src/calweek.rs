use chrono::{Datelike, Local, NaiveDate, Weekday};
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

pub fn get_calweek_by_date(date: NaiveDate) -> CalWeekResult {
    let iso_week = date.iso_week();
    let year_str = format!("{:02}", iso_week.year().rem_euclid(100));
    let week_number = iso_week.week();
    let calweek = format!("{year_str}{week_number:02}");

    CalWeekResult {
        calweek,
        year: year_str,
        week_number,
    }
}

pub fn get_current_calweek() -> CalWeekResult {
    get_calweek_by_date(Local::now().date_naive())
}

#[derive(Serialize, Debug)]
pub struct CalWeekDayResult {
    pub weekday: String,
    pub date: String,
}

impl fmt::Display for CalWeekDayResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.weekday, self.date)
    }
}

pub fn get_day_from_calweekday(calweek5: &str) -> Result<CalWeekDayResult, String> {
    if calweek5.len() != 5 || !calweek5.chars().all(|c| c.is_ascii_digit()) {
        return Err(String::from("CalWeekDay must be a 5-digit value (e.g., 26262)"));
    }

    let year = calweek5[..2]
        .parse::<i32>()
        .map_err(|_| String::from("CalWeekDay year is not valid"))?
        + 2000;
    let week_number = calweek5[2..4]
        .parse::<u32>()
        .map_err(|_| String::from("CalWeekDay week number is not valid"))?;
    let day_digit = calweek5[4..]
        .parse::<u32>()
        .map_err(|_| String::from("CalWeekDay day is not valid"))?;

    let (weekday_name, weekday) = match day_digit {
        1 => ("Monday", Weekday::Mon),
        2 => ("Tuesday", Weekday::Tue),
        3 => ("Wednesday", Weekday::Wed),
        4 => ("Thursday", Weekday::Thu),
        5 => ("Friday", Weekday::Fri),
        6 => ("Saturday", Weekday::Sat),
        7 => ("Sunday", Weekday::Sun),
        _ => return Err(String::from("Day digit must be between 1 (Monday) and 7 (Sunday)")),
    };

    let date = NaiveDate::from_isoywd_opt(year, week_number, weekday)
        .ok_or_else(|| String::from("CalWeekDay is out of ISO range"))?;

    Ok(CalWeekDayResult {
        weekday: weekday_name.to_string(),
        date: date.format("%d.%m.%Y").to_string(),
    })
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
#[path = "calweek_tests.rs"]
mod tests;
