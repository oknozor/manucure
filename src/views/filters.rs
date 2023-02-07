use chrono::{Datelike, NaiveDateTime, Utc};
use chrono::format::Fixed::RFC2822;

pub fn date_format(date: &NaiveDateTime) -> askama::Result<String> {
    let date = date.format("%F");
    let date = date.to_string();
    Ok(date)
}

