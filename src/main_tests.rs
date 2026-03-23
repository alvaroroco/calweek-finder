use super::{
    classify_input, handle_date_conversion, handle_week_conversion, should_start_interactive_for, Cli,
    InputKind,
};
use clap::Parser;

#[test]
fn classifies_today_input() {
    assert_eq!(classify_input("today"), InputKind::Today);
    assert_eq!(classify_input("TODAY"), InputKind::Today);
}

#[test]
fn classifies_week_input() {
    assert_eq!(classify_input("2348"), InputKind::Week);
    assert_eq!(classify_input("23482"), InputKind::Week);
}

#[test]
fn classifies_date_input() {
    assert_eq!(classify_input("2023-11-26"), InputKind::Date);
    assert_eq!(classify_input("abcd"), InputKind::Date);
}

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
fn date_conversion_returns_error_for_invalid_date() {
    let result = handle_date_conversion("2023-13-40", false);
    assert!(result.is_err());
}

#[test]
fn week_conversion_returns_error_for_invalid_week() {
    let result = handle_week_conversion("2454", false);
    assert!(result.is_err());
}

#[test]
fn week_conversion_returns_error_for_invalid_day_digit() {
    let result = handle_week_conversion("23480", false);
    assert!(result.is_err());
}

#[test]
fn only_4_and_5_digit_numbers_are_treated_as_weeks() {
    assert_ne!(classify_input("123"), InputKind::Week);
    assert_ne!(classify_input("123456"), InputKind::Week);
}

#[test]
fn interactive_mode_requires_tty_on_stdin_and_stdout() {
    assert!(should_start_interactive_for(true, true));
    assert!(!should_start_interactive_for(true, false));
    assert!(!should_start_interactive_for(false, true));
    assert!(!should_start_interactive_for(false, false));
}
