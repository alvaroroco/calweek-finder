mod dates;
mod calweek;
mod ui;

use clap::Parser;
use serde_json::json;
use std::env;
use std::io::{self, IsTerminal};
use std::process::ExitCode;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,

    /// Date to convert to CalWeek (e.g., 2023-11-26)
    #[arg(long, short, conflicts_with_all = ["week", "input"])]
    date: Option<String>,

    /// CalWeek (YYWW) to convert to Date, where YY is 2000-2099 (e.g., 2348)
    #[arg(long, short, conflicts_with_all = ["date", "input"])]
    week: Option<String>,

    /// Value to auto-detect: 4 digits = CalWeek, date format = Date
    #[arg(index = 1)]
    value: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum InputKind {
    Today,
    Week,
    Date,
}

fn main() -> ExitCode {
    if env::args().len() <= 1 {
        if should_start_interactive() {
            ui::questions();
            return ExitCode::SUCCESS;
        }

        eprintln!("No arguments provided in a non-interactive environment. Use --help.");
        return ExitCode::from(2);
    }

    let cli = Cli::parse();
    let result = if let Some(date_str) = cli.date {
        handle_date_conversion(&date_str, cli.json)
    } else if let Some(week_str) = cli.week {
        handle_week_conversion(&week_str, cli.json);
    } else if let Some(value) = cli.value {
        if value.len() == 4 {
            handle_week_conversion(&value, cli.json);
        } else if dates::str_to_date(&value).is_some() {
            handle_date_conversion(&value, cli.json);
        } else {
            eprintln!("Could not detect format for '{}'. Use --date or --week explicitly.", value);
        }
    } else {
        match cli.command {
            Some(Commands::Today) => handle_today(cli.json),
            None => {}
        }
    }
}

fn handle_date_conversion(date_str: &str, json_output: bool) -> Result<(), String> {
    match dates::str_to_date(date_str) {
        Some(date) => {
            let result = calweek::get_calweek_by_date(date);
            print_success(&result, json_output);
            Ok(())
        }
        None => Err(String::from("Invalid date")),
    }
}

fn handle_week_conversion(week_str: &str, json_output: bool) -> Result<(), String> {
    match calweek::get_monday_and_sunday(week_str) {
        Ok(result) => {
            print_success(&result, json_output);
            Ok(())
        }
        Err(error) => Err(error),
    }
}

fn handle_today(json_output: bool) -> Result<(), String> {
    let result = calweek::get_current_calweek();
    print_success(&result, json_output);
    Ok(())
}

fn print_success<T: serde::Serialize + std::fmt::Display>(value: &T, json_output: bool) {
    if json_output {
        println!("{}", json!(value));
    } else {
        println!("{value}");
    }
}

fn print_error(error: &str, json_output: bool) {
    if json_output {
        println!("{}", json!({ "error": error }));
    } else {
        eprintln!("{error}");
    }
}

#[cfg(test)]
mod tests {
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
    fn only_digits_with_len_four_are_treated_as_weeks() {
        assert_ne!(classify_input("123"), InputKind::Week);
        assert_ne!(classify_input("12345"), InputKind::Week);
    }

    #[test]
    fn interactive_mode_requires_tty_on_stdin_and_stdout() {
        assert!(should_start_interactive_for(true, true));
        assert!(!should_start_interactive_for(true, false));
        assert!(!should_start_interactive_for(false, true));
        assert!(!should_start_interactive_for(false, false));
    }
}
