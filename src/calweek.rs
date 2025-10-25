use chrono::{Datelike, NaiveDate, Utc, Weekday};

fn get_week_number(date_obj: NaiveDate) -> u32 {
    date_obj.iso_week().week()
}

pub fn get_calweek_by_date(date: NaiveDate) -> String {
    let year: String = format!("{:02}", date.year() % 100);
    let week_number: u32 = get_week_number(date);

    String::from(format!("CalWeek: {}{}", year, week_number))
}

pub fn get_current_calweek() -> String {
    get_calweek_by_date(Utc::now().date_naive())
}

pub fn get_monday_and_sunday(calweek: &str) -> String {
    let year: i32 = calweek[..2].parse::<i32>().unwrap() + 2000;
    let week_number: u32 = calweek[calweek.len() - 2..].parse::<u32>().unwrap();

    let monday_date: NaiveDate = NaiveDate::from_isoywd_opt(year, week_number, Weekday::Mon).unwrap();
    let sunday_date: NaiveDate = NaiveDate::from_isoywd_opt(year, week_number, Weekday::Sun).unwrap();

    let str_monday: String = monday_date.format("%d.%m.%Y").to_string();
    let str_sunday: String = sunday_date.format("%d.%m.%Y").to_string();

    String::from(format!("Monday: {}\nSunday: {}", str_monday, str_sunday))
}
