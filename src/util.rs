use chrono::naive::NaiveDate;

pub fn parse_date(line: &str) -> Option<NaiveDate> {
    if let Ok(date) = NaiveDate::parse_from_str(line, "%F") {
        Some(date)
    } else {
        None
    }
}
