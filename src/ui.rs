use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::dates;
use crate::calweek;

pub fn questions() {
    let selections: &[&str; 2] = &["Get date by calweek", "Get calweek by date"];

    let selection = match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an action")
        .default(0)
        .items(&selections[..])
        .interact()
    {
        Ok(choice) => choice,
        Err(error) => {
            eprintln!("Failed to read selection: {error}");
            return;
        }
    };

    match selections[selection] {
        "Get calweek by date" => {
            let user_date = match Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter date (YYYY-MM-DD, DD/MM/YYYY, MM/DD/YYYY, DD-MM-YYYY, DD.MM.YYYY; ambiguous slash dates like 01/02/2023 are rejected)")
                .interact()
            {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("Failed to read date: {error}");
                    return;
                }
            };

            match dates::str_to_date(&user_date) {
                Some(date) => println!("{}", calweek::get_calweek_by_date(date)),
                None => println!("Invalid date"),
            }
        }
        "Get date by calweek" => {
            let user_calweek: String = match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the calweek")
                .interact_text()
            {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("Failed to read calweek: {error}");
                    return;
                }
            };

            match calweek::get_monday_and_sunday(&user_calweek) {
                Ok(result) => println!("{result}"),
                Err(error) => println!("{error}"),
            }
        }
        _ => unreachable!(),
    }
}
