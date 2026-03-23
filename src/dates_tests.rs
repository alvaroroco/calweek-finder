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

#[test]
fn rejects_ambiguous_slash_dates() {
    assert!(str_to_date("01/02/2023").is_none());
    assert!(str_to_date("12/11/2023").is_none());
}

#[test]
fn accepts_unambiguous_slash_dates() {
    assert!(str_to_date("26/11/2023").is_some());
    assert!(str_to_date("11/26/2023").is_some());
    assert!(str_to_date("02/02/2023").is_some());
}
