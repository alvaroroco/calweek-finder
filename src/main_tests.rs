use super::{format_output, should_start_interactive_for, Cli};
use crate::app::AppOutput;
use crate::calweek::{CalWeekDayResult, CalWeekResult, WeekRangeResult};
use clap::Parser;

#[test]
fn rejects_date_and_week_together() {
    let parsed = Cli::try_parse_from(["calweek_finder", "--date", "2023-11-26", "--week", "2348"]);
    assert!(parsed.is_err());
}

#[test]
fn rejects_explicit_flag_and_positional_input_together() {
    let parsed = Cli::try_parse_from(["calweek_finder", "--date", "2023-11-26", "today"]);
    assert!(parsed.is_err());
}

#[test]
fn interactive_mode_requires_tty_on_stdin_and_stdout() {
    assert!(should_start_interactive_for(true, true));
    assert!(!should_start_interactive_for(true, false));
    assert!(!should_start_interactive_for(false, true));
    assert!(!should_start_interactive_for(false, false));
}

#[test]
fn formats_calweek_output() {
    let output = AppOutput::Calweek(CalWeekResult {
        calweek: "2347".to_string(),
        year: "23".to_string(),
        week_number: 47,
    });
    assert_eq!(format_output(&output), "CalWeek: 2347");
}

#[test]
fn formats_week_range_output() {
    let output = AppOutput::WeekRange(WeekRangeResult {
        monday: "27.11.2023".to_string(),
        sunday: "03.12.2023".to_string(),
    });
    assert_eq!(format_output(&output), "Monday: 27.11.2023\nSunday: 03.12.2023");
}

#[test]
fn formats_week_day_output() {
    let output = AppOutput::WeekDay(CalWeekDayResult {
        weekday: "Tuesday".to_string(),
        date: "28.11.2023".to_string(),
    });
    assert_eq!(format_output(&output), "Tuesday: 28.11.2023");
}
