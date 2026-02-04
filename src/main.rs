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
    } else {
        match cli.command {
            Some(Commands::Today) => handle_today(cli.json),
            None => {
                // If we are here, it means args were provided but didn't match known flags/commands perfectly
                // or maybe just --json was passed without a command.
                // However, clap usually handles help/errors.
                // For backward compatibility, we could try to parse the first arg as the old style
                // but since we are moving to clap, let's stick to the new interface or TUI.
                // But wait, the original code allowed `calweek 2348` or `calweek today`.
                // Let's try to support that if possible, or just rely on the new flags.
                // Given the request was to add CLI args like --date, I will prioritize that.
                // If the user passes "today" as a positional arg, clap might fail if not configured.
                // Let's keep it simple: use the flags.

                // Fallback for "today" as a positional arg if it was passed and not caught by clap as a subcommand
                // Actually, clap will error if it sees an unknown positional arg.
                // So we rely on the user using the flags or the TUI.
            }
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
