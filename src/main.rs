mod dates;
mod calweek;
mod ui;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        ui::questions();
        return;
    }

    let input_arg: &String = &args[1];

    if input_arg == "today" {
        println!("{}", calweek::get_current_calweek());
    } else if input_arg.len() != 4 {
        match dates::str_to_date(input_arg) {
            Some(date) => println!("{}", calweek::get_calweek_by_date(date)),
            None => println!("Date no valid"),
        }
    } else {
        println!("{}", calweek::get_monday_and_sunday(input_arg))
    }
}
