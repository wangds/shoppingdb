use chrono::naive::NaiveDate;

pub fn today() -> String {
    chrono::offset::Local::now().format("%F").to_string()
}

pub fn parse_date(line: &str) -> Option<NaiveDate> {
    if let Ok(date) = NaiveDate::parse_from_str(line, "%F") {
        Some(date)
    } else {
        None
    }
}

pub fn format_price(price: i64) -> String {
    format!("{}.{:02}", price / 100, price % 100)
}

pub fn parse_price(line: &str) -> Option<i64> {
    let dollars: &str;
    let cents: Option<i64>;

    if let Some((d, c)) = line.split_once('.') {
        dollars = d;
        cents = match c.len() {
            0 => Some(0i64),
            1 => c.parse::<i64>().ok().map(|c| c * 10),
            2 => c.parse::<i64>().ok(),
            _ => None,
        };
    } else {
        dollars = line;
        cents = Some(0i64)
    }

    cents
        .zip(dollars.parse::<i64>().ok())
        .map(|(c, d)| 100 * d + c)
}
