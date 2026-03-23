mod dates;
mod calweek;
mod ui;

use clap::{Parser, Subcommand};
use serde_json::json;
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,

    /// Date to convert to CalWeek (e.g., 2023-11-26)
    #[arg(long, short)]
    date: Option<String>,

    /// CalWeek to convert to Date (e.g., 2348)
    #[arg(long, short)]
    week: Option<String>,

    /// Value to auto-detect: 4 digits = CalWeek, date format = Date
    #[arg(index = 1)]
    value: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Get current CalWeek
    Today,
}

fn main() {
    // Check if any arguments were provided (other than the program name)
    if env::args().len() <= 1 {
        ui::questions();
        return;
    }

    let cli = Cli::parse();

    // Handle flags directly if no subcommand is used but flags are present
    if let Some(date_str) = cli.date {
        handle_date_conversion(&date_str, cli.json);
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

fn handle_date_conversion(date_str: &str, json_output: bool) {
    match dates::str_to_date(date_str) {
        Some(date) => {
            let result = calweek::get_calweek_by_date(date);
            if json_output {
                println!("{}", json!(result));
            } else {
                println!("{}", result);
            }
        }
        None => {
            if json_output {
                println!("{}", json!({ "error": "Date not valid" }));
            } else {
                println!("Date no valid");
            }
        }
    }
}

fn handle_week_conversion(week_str: &str, json_output: bool) {
    // Basic validation could be added here or inside calweek
    // For now, we trust calweek's unwrap (which we should fix later as per my advice, but for now we stick to the plan)
    let result = calweek::get_monday_and_sunday(week_str);
    if json_output {
        println!("{}", json!(result));
    } else {
        println!("{}", result);
    }
}

fn handle_today(json_output: bool) {
    let result = calweek::get_current_calweek();
    if json_output {
        println!("{}", json!(result));
    } else {
        println!("{}", result);
    }
}
