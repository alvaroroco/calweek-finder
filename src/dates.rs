use chrono::NaiveDate;

pub fn str_to_date(string_date: &str) -> Option<NaiveDate> {
    if is_ambiguous_slash_date(string_date) {
        return None;
    }

    let formatos: Vec<&str> = vec!["%Y-%m-%d", "%d/%m/%Y", "%m/%d/%Y", "%d-%m-%Y", "%d.%m.%Y"];

    for formato in formatos {
        if let Ok(fecha) = NaiveDate::parse_from_str(string_date, formato) {
            return Some(fecha)
        }
    }

    None
}

fn is_ambiguous_slash_date(input: &str) -> bool {
    if !input.contains('/') {
        return false;
    }

    let parts: Vec<&str> = input.split('/').collect();
    if parts.len() != 3 {
        return false;
    }

    let first = match parts[0].parse::<u32>() {
        Ok(value) => value,
        Err(_) => return false,
    };
    let second = match parts[1].parse::<u32>() {
        Ok(value) => value,
        Err(_) => return false,
    };

    first <= 12 && second <= 12 && first != second
}

#[cfg(test)]
#[path = "dates_tests.rs"]
mod tests;
