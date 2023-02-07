use chrono::NaiveDateTime;
use estimated_read_time::Options;
use url::Url;

pub fn date_format(date: &NaiveDateTime) -> askama::Result<String> {
    let date = date.format("%v");
    let date = date.to_string();
    Ok(date)
}

pub fn read_time<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let read_time = estimated_read_time::text(&s.to_string(), &Options::default());
    let minutes = read_time.seconds() / 60;
    if minutes == 0 {
        Ok(format!("Less than a minute"))
    } else {
        Ok(format!("{minutes} min"))
    }
}

pub fn domain<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let url = Url::parse(&s.to_string()).map_err(|err| askama::Error::Custom(Box::new(err)))?;

    let domain = url.domain().expect("Article url should be valid");

    Ok(domain.to_string())
}
