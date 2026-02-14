use chrono::NaiveDate;

pub fn str_to_date(string_date: &str) -> Option<NaiveDate> {
    let formatos: Vec<&str> = vec!["%Y-%m-%d", "%d/%m/%Y", "%m/%d/%Y", "%d-%m-%Y", "%d.%m.%Y"];

    for formato in formatos {
        if let Ok(fecha) = NaiveDate::parse_from_str(string_date, formato) {
            return Some(fecha)
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::str_to_date;

    #[test]
    fn parses_supported_formats() {
        assert!(str_to_date("2023-11-26").is_some());
        assert!(str_to_date("26/11/2023").is_some());
        assert!(str_to_date("11/26/2023").is_some());
        assert!(str_to_date("26-11-2023").is_some());
        assert!(str_to_date("26.11.2023").is_some());
    }

    #[test]
    fn rejects_invalid_date() {
        assert!(str_to_date("2023-13-40").is_none());
        assert!(str_to_date("not-a-date").is_none());
    }
}
