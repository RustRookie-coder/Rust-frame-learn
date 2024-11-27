use chrono::{DateTime, NaiveDateTime, Utc};

pub fn str_to_date_time(str: &str) -> DateTime<Utc> {
    DateTime::from_utc(NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M:%S")
                           .expect("Failed to parse time string"), Utc)
}