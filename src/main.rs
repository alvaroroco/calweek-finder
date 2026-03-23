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

    /// CalWeek to convert to Date: 4 digits YYWW for week range (e.g., 2348), 5 digits YYWWD for specific day (e.g., 23482)
    #[arg(long, short, conflicts_with_all = ["date", "input"])]
    week: Option<String>,

    /// Input value: "today", a calweek like 2348 or 26262, or a date
    #[arg(conflicts_with_all = ["date", "week"])]
    input: Option<String>,
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
        handle_week_conversion(&week_str, cli.json)
    } else if let Some(input) = cli.input {
        handle_input_conversion(&input, cli.json)
    } else {
        Ok(())
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            print_error(&error, cli.json);
            ExitCode::from(1)
        }
    }
}

fn should_start_interactive() -> bool {
    should_start_interactive_for(io::stdin().is_terminal(), io::stdout().is_terminal())
}

fn should_start_interactive_for(stdin_is_tty: bool, stdout_is_tty: bool) -> bool {
    stdin_is_tty && stdout_is_tty
}

fn handle_input_conversion(input: &str, json_output: bool) -> Result<(), String> {
    match classify_input(input) {
        InputKind::Today => handle_today(json_output),
        InputKind::Week => handle_week_conversion(input, json_output),
        InputKind::Date => handle_date_conversion(input, json_output),
    }
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
    if week_str.len() == 5 {
        match calweek::get_day_from_calweekday(week_str) {
            Ok(result) => {
                print_success(&result, json_output);
                Ok(())
            }
            Err(error) => Err(error),
        }
    } else {
        match calweek::get_monday_and_sunday(week_str) {
            Ok(result) => {
                print_success(&result, json_output);
                Ok(())
            }
            Err(error) => Err(error),
        }
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
#[path = "main_tests.rs"]
mod tests;
