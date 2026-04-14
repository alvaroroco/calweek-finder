mod app;
mod calweek;
mod dates;
mod ui;

use app::AppOutput;
use clap::{Parser, ValueEnum};
use serde_json::json;
use std::env;
use std::io::{self, IsTerminal};
use std::process::ExitCode;

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    /// Human-readable text (default)
    Text,
    /// Machine-readable JSON array
    Json,
    /// CSV with header row (useful for multiple inputs)
    Csv,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(after_help = "\
EXAMPLES:
  calweek today                         Current calweek
  calweek 2348                          Date range for calweek 2348
  calweek 23482                         Specific day (2=Tuesday) of calweek 2348
  calweek 2023-11-26                    Calweek for a specific date
  calweek --date 2023-11-26 -o json     JSON output for scripting
  calweek --week 2348 -o json | jq .    Pipe JSON to jq")]
struct Cli {
    /// Output format
    #[arg(long, short = 'o', value_enum, default_value = "text", global = true)]
    output: OutputFormat,

    /// [Deprecated] Use -o json instead
    #[arg(long, hide = true, global = true)]
    json: bool,

    /// Date to convert to CalWeek (e.g., 2023-11-26)
    #[arg(long, short, conflicts_with_all = ["week", "input"])]
    date: Option<String>,

    /// CalWeek to convert to Date: 4 digits YYWW for week range (e.g., 2348), 5 digits YYWWD for specific day (e.g., 23482)
    #[arg(long, short, conflicts_with_all = ["date", "input"])]
    week: Option<String>,

    /// Input values: "today", calweeks like 2348 or 23482, or dates — all must be the same type
    #[arg(conflicts_with_all = ["date", "week"], num_args(1..))]
    input: Vec<String>,
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
    let format = if cli.json { OutputFormat::Json } else { cli.output };

    let result: Result<Vec<AppOutput>, String> = if let Some(date_str) = cli.date {
        app::process_date(&date_str).map(|o| vec![o])
    } else if let Some(week_str) = cli.week {
        app::process_week(&week_str).map(|o| vec![o])
    } else if !cli.input.is_empty() {
        app::process_inputs(&cli.input)
    } else {
        return ExitCode::SUCCESS;
    };

    match result {
        Ok(outputs) => {
            print_outputs(&outputs, &format);
            ExitCode::SUCCESS
        }
        Err(error) => {
            print_error(&error, &format);
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

fn csv_header(output: &AppOutput) -> &'static str {
    match output {
        AppOutput::Calweek(_)   => "calweek,year,week_number",
        AppOutput::WeekRange(_) => "monday,sunday",
        AppOutput::WeekDay(_)   => "weekday,date",
    }
}

fn csv_row(output: &AppOutput) -> String {
    match output {
        AppOutput::Calweek(r)   => format!("{},{},{}", r.calweek, r.year, r.week_number),
        AppOutput::WeekRange(r) => format!("{},{}", r.monday, r.sunday),
        AppOutput::WeekDay(r)   => format!("{},{}", r.weekday, r.date),
    }
}

fn print_outputs(outputs: &[AppOutput], format: &OutputFormat) {
    match format {
        OutputFormat::Text => {
            let parts: Vec<String> = outputs.iter().map(format_output).collect();
            println!("{}", parts.join("\n"));
        }
        OutputFormat::Json => {
            let json_vals: Vec<_> = outputs.iter().map(|o| match o {
                AppOutput::Calweek(r)   => json!(r),
                AppOutput::WeekRange(r) => json!(r),
                AppOutput::WeekDay(r)   => json!(r),
            }).collect();
            println!("{}", json!(json_vals));
        }
        OutputFormat::Csv => {
            if let Some(first) = outputs.first() {
                println!("{}", csv_header(first));
                for o in outputs {
                    println!("{}", csv_row(o));
                }
            }
        }
    }
}

fn print_error(error: &str, format: &OutputFormat) {
    if matches!(format, OutputFormat::Json) {
        println!("{}", json!({ "error": error }));
    } else {
        eprintln!("{error}");
    }
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod tests;
