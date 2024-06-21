use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, ParseError, TimeZone, Utc};

pub struct Date;

impl Date {
    pub fn from_date_str(date_str: &str) -> Result<DateTime<Utc>, ParseError> {
        let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
        let naive_time = match NaiveTime::from_hms_opt(0, 0, 0) {
            Some(time) => time,
            None => {
                // TODO: Is there a better way to throw ParseError?
                NaiveDate::parse_from_str("break", "break")?;
                return Ok(Utc::now());
            }
        };
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
        let utc = Utc::from_utc_datetime(&Utc, &naive_datetime);
        Ok(utc)
    }

    pub fn to_date_str(time: DateTime<Utc>) -> String {
        format!("{:04}-{:02}-{:02}", time.year(), time.month(), time.day())
    }

    pub fn get_local_date_str() -> String {
        Self::to_date_str(Utc::now())
    }
}
