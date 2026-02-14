mod dates;
mod calweek;
mod ui;

use clap::Parser;
use serde_json::json;
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,

    /// Date to convert to CalWeek (e.g., 2023-11-26)
    #[arg(long, short, conflicts_with_all = ["week", "input"])]
    date: Option<String>,

    /// CalWeek to convert to Date (e.g., 2348)
    #[arg(long, short, conflicts_with_all = ["date", "input"])]
    week: Option<String>,

    /// Input value: "today", a calweek like 2348, or a date
    #[arg(conflicts_with_all = ["date", "week"])]
    input: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum InputKind {
    Today,
    Week,
    Date,
}

fn main() {
    if env::args().len() <= 1 {
        ui::questions();
        return;
    }

    let cli = Cli::parse();

    if let Some(date_str) = cli.date {
        handle_date_conversion(&date_str, cli.json);
    } else if let Some(week_str) = cli.week {
        handle_week_conversion(&week_str, cli.json);
    } else if let Some(input) = cli.input {
        handle_input_conversion(&input, cli.json);
    }
}

fn handle_input_conversion(input: &str, json_output: bool) {
    match classify_input(input) {
        InputKind::Today => handle_today(json_output),
        InputKind::Week => handle_week_conversion(input, json_output),
        InputKind::Date => handle_date_conversion(input, json_output),
    }
}

fn classify_input(input: &str) -> InputKind {
    if input.eq_ignore_ascii_case("today") {
        InputKind::Today
    } else if input.len() == 4 && input.chars().all(|c| c.is_ascii_digit()) {
        InputKind::Week
    } else {
        InputKind::Date
    }
}

fn handle_date_conversion(date_str: &str, json_output: bool) {
    match dates::str_to_date(date_str) {
        Some(date) => {
            let result = calweek::get_calweek_by_date(date);
            if json_output {
                println!("{}", json!(result));
            } else {
                println!("{result}");
            }
        }
        None => {
            if json_output {
                println!("{}", json!({ "error": "Invalid date" }));
            } else {
                println!("Invalid date");
            }
        }
    }
}

fn handle_week_conversion(week_str: &str, json_output: bool) {
    match calweek::get_monday_and_sunday(week_str) {
        Ok(result) => {
            if json_output {
                println!("{}", json!(result));
            } else {
                println!("{result}");
            }
        }
        Err(error) => {
            if json_output {
                println!("{}", json!({ "error": error }));
            } else {
                println!("{error}");
            }
        }
    }
}

fn handle_today(json_output: bool) {
    let result = calweek::get_current_calweek();
    if json_output {
        println!("{}", json!(result));
    } else {
        println!("{result}");
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_input, Cli, InputKind};
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
}
