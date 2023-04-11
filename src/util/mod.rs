use crate::config::Config;
use chrono::prelude::*;

pub mod types;

pub fn is_auth(config: Config) -> bool {
    config.get_user().is_some()
}

pub fn dateize_timestamp(timestamp: String) -> Option<String> {
    if let Ok(ts) = timestamp.parse::<i32>() {
        let datetime = NaiveDateTime::from_timestamp_opt(ts as i64, 0)?;
        let datetime: DateTime<Utc> = DateTime::from_utc(datetime, Utc);
        return Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string());
    }
    None
}