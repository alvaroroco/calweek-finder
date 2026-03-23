mod app;
mod calweek;
mod dates;
mod ui;

use app::AppOutput;
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
    let json_output = cli.json;

    let result = if let Some(date_str) = cli.date {
        app::process_date(&date_str)
    } else if let Some(week_str) = cli.week {
        app::process_week(&week_str)
    } else if let Some(input) = cli.input {
        app::process_input(&input)
    } else {
        return ExitCode::SUCCESS;
    };

    match result {
        Ok(output) => {
            print_output(&output, json_output);
            ExitCode::SUCCESS
        }
        Err(error) => {
            print_error(&error, json_output);
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

pub fn format_output(output: &AppOutput) -> String {
    match output {
        AppOutput::Calweek(r)   => format!("CalWeek: {}", r.calweek),
        AppOutput::WeekRange(r) => format!("Monday: {}\nSunday: {}", r.monday, r.sunday),
        AppOutput::WeekDay(r)   => format!("{}: {}", r.weekday, r.date),
    }
}

fn print_output(output: &AppOutput, json_output: bool) {
    if json_output {
        let json_val = match output {
            AppOutput::Calweek(r)   => json!(r),
            AppOutput::WeekRange(r) => json!(r),
            AppOutput::WeekDay(r)   => json!(r),
        };
        println!("{json_val}");
    } else {
        println!("{}", format_output(output));
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
