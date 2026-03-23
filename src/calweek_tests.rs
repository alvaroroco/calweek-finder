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
fn uses_iso_year_for_boundary_dates() {
    let date = NaiveDate::from_ymd_opt(2021, 1, 1).expect("valid date");
    let result = get_calweek_by_date(date);

    assert_eq!(result.calweek, "2053");
    assert_eq!(result.week_number, 53);
    assert_eq!(result.year, "20");
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

#[test]
fn converts_5digit_calweek_to_specific_day() {
    let result = get_day_from_calweekday("23482").expect("valid calweekday");
    assert_eq!(result.weekday, "Tuesday");
    assert_eq!(result.date, "28.11.2023");
}

#[test]
fn rejects_invalid_day_digit() {
    assert!(get_day_from_calweekday("23480").is_err());
    assert!(get_day_from_calweekday("23488").is_err());
}

#[test]
fn rejects_non_numeric_calweekday() {
    assert!(get_day_from_calweekday("2348x").is_err());
}
