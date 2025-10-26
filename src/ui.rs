use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::dates;
use crate::calweek;

pub fn questions() {
    let selections: &[&str; 2] = &["Get date by calweek", "Get calweek by date"];

    let selection: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an action")
        .default(0).items(&selections[..])
        .interact().unwrap();

    match selections[selection] {
        "Get calweek by date" => {
            let user_date: String = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the date (Day-Month-Year)")
                .interact().unwrap();

            match dates::str_to_date(&user_date) {
                Some(date) => println!("{}", calweek::get_calweek_by_date(date)),
                None => println!("Date no valid"),
            }
        }
        "Get date by calweek" => {
            let user_calweek: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the calweek")
                .interact_text()
                .unwrap();

            println!("{}", calweek::get_monday_and_sunday(&user_calweek));
        }
        _ => unreachable!(),
    }
}
