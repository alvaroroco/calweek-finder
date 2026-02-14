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
    #[arg(long, short)]
    date: Option<String>,

    /// CalWeek to convert to Date (e.g., 2348)
    #[arg(long, short)]
    week: Option<String>,

    /// Input value: "today", a calweek like 2348, or a date
    input: Option<String>,
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
    if input.eq_ignore_ascii_case("today") {
        handle_today(json_output);
    } else if input.len() == 4 && input.chars().all(|c| c.is_ascii_digit()) {
        handle_week_conversion(input, json_output);
    } else {
        handle_date_conversion(input, json_output);
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
