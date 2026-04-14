use crate::calweek::{self, CalWeekDayResult, CalWeekResult, WeekRangeResult};
use crate::dates;

#[derive(Debug)]
pub enum AppOutput {
    Calweek(CalWeekResult),
    WeekRange(WeekRangeResult),
    WeekDay(CalWeekDayResult),
}

enum InputKind {
    Today,
    Week,
    Date,
}

fn classify_input(input: &str) -> InputKind {
    if input.eq_ignore_ascii_case("today") {
        InputKind::Today
    } else if (input.len() == 4 || input.len() == 5) && input.chars().all(|c| c.is_ascii_digit()) {
        InputKind::Week
    } else {
        InputKind::Date
    }
}

pub fn process_input(input: &str) -> Result<AppOutput, String> {
    match classify_input(input) {
        InputKind::Today => Ok(process_today()),
        InputKind::Week => process_week(input),
        InputKind::Date => process_date(input),
    }
}

pub fn process_date(date_str: &str) -> Result<AppOutput, String> {
    match dates::str_to_date(date_str) {
        Some(date) => Ok(AppOutput::Calweek(calweek::get_calweek_by_date(date))),
        None => Err(format!(
            "Cannot parse date \"{date_str}\". Supported formats: YYYY-MM-DD, DD.MM.YYYY, DD-MM-YYYY, DD/MM/YYYY, MM/DD/YYYY\nNote: ambiguous slash dates (e.g. 01/02/2023) are rejected — use DD.MM.YYYY or YYYY-MM-DD"
        )),
    }
}

pub fn process_week(week_str: &str) -> Result<AppOutput, String> {
    if week_str.len() == 5 {
        calweek::get_day_from_calweekday(week_str).map(AppOutput::WeekDay)
    } else {
        calweek::get_monday_and_sunday(week_str).map(AppOutput::WeekRange)
    }
}

pub fn process_today() -> AppOutput {
    AppOutput::Calweek(calweek::get_current_calweek())
}

#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;
