use chrono::{DateTime, Local, TimeZone, Utc};

use crate::{
    constants::DEFAULT_DATETIME_FORMAT_STR,
    time::{LocalTimeTool, UtcTimeTool},
};

pub trait CurrentTime<Tz: TimeZone>
where
    Tz::Offset: std::fmt::Display,
{
    fn get_current() -> DateTime<Tz>;

    fn get_unix_time() -> u64 {
        Self::get_current().timestamp_millis() as u64
    }

    fn get_date_time() -> String {
        Self::get_current()
            .format(DEFAULT_DATETIME_FORMAT_STR)
            .to_string()
    }
}

impl CurrentTime<Local> for LocalTimeTool {
    fn get_current() -> DateTime<Local> {
        Local::now()
    }
}

impl CurrentTime<Utc> for UtcTimeTool {
    fn get_current() -> DateTime<Utc> {
        Utc::now()
    }
}

pub enum DurationInterval {
    Seconds1,
    Minutes1,
    Hours1,
    Days1,
    Weeks1,
    Months1,
    Years1,
}

pub trait GetDuration {
    fn get_duration(&self, interval: DurationInterval) -> (u64, u64);
}

impl GetDuration for UtcTimeTool {
    fn get_duration(&self, interval: DurationInterval) -> (u64, u64) {
        let current = Self::get_current();
        let start = match interval {
            DurationInterval::Seconds1 => current - chrono::Duration::seconds(1),
            DurationInterval::Minutes1 => current - chrono::Duration::minutes(1),
            DurationInterval::Hours1 => current - chrono::Duration::hours(1),
            DurationInterval::Days1 => current - chrono::Duration::days(1),
            DurationInterval::Weeks1 => current - chrono::Duration::weeks(1),
            DurationInterval::Months1 => current - chrono::Duration::days(30),
            DurationInterval::Years1 => current - chrono::Duration::days(365),
        };
        (
            start.timestamp_millis() as u64,
            current.timestamp_millis() as u64,
        )
    }
}
