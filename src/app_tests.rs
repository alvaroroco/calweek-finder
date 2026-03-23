use super::{process_date, process_input, process_today, process_week, AppOutput};

#[test]
fn process_today_returns_calweek() {
    assert!(matches!(process_today(), AppOutput::Calweek(_)));
}

#[test]
fn process_date_returns_calweek_for_valid_date() {
    let result = process_date("2023-11-26").expect("valid date");
    match result {
        AppOutput::Calweek(r) => assert_eq!(r.calweek, "2347"),
        _ => panic!("expected Calweek variant"),
    }
}

#[test]
fn process_date_returns_error_for_invalid_date() {
    assert!(process_date("not-a-date").is_err());
    assert!(process_date("2023-13-40").is_err());
}

#[test]
fn process_week_returns_range_for_4_digits() {
    let result = process_week("2348").expect("valid calweek");
    match result {
        AppOutput::WeekRange(r) => {
            assert_eq!(r.monday, "27.11.2023");
            assert_eq!(r.sunday, "03.12.2023");
        }
        _ => panic!("expected WeekRange variant"),
    }
}

#[test]
fn process_week_returns_day_for_5_digits() {
    let result = process_week("23482").expect("valid calweekday");
    match result {
        AppOutput::WeekDay(r) => {
            assert_eq!(r.weekday, "Tuesday");
            assert_eq!(r.date, "28.11.2023");
        }
        _ => panic!("expected WeekDay variant"),
    }
}

#[test]
fn process_week_returns_error_for_invalid_week() {
    assert!(process_week("2454").is_err());
}

#[test]
fn process_week_returns_error_for_invalid_day_digit() {
    assert!(process_week("23480").is_err());
    assert!(process_week("23488").is_err());
}

#[test]
fn process_input_classifies_today() {
    assert!(matches!(process_input("today"), Ok(AppOutput::Calweek(_))));
    assert!(matches!(process_input("TODAY"), Ok(AppOutput::Calweek(_))));
}

#[test]
fn process_input_classifies_4digit_as_week_range() {
    assert!(matches!(process_input("2348"), Ok(AppOutput::WeekRange(_))));
}

#[test]
fn process_input_classifies_5digit_as_week_day() {
    assert!(matches!(process_input("23482"), Ok(AppOutput::WeekDay(_))));
}

#[test]
fn process_input_classifies_date_string() {
    assert!(matches!(process_input("2023-11-26"), Ok(AppOutput::Calweek(_))));
}

#[test]
fn process_input_rejects_3_and_6_digit_strings() {
    assert!(process_input("123").is_err());
    assert!(process_input("123456").is_err());
}
