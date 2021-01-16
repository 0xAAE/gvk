use chrono::{DateTime, Local, NaiveDateTime, Utc};

pub fn local_from_timestamp(timestamp: i64) -> DateTime<Local> {
    utc_from_timestamp(timestamp).with_timezone(&Local)
}

pub fn utc_from_timestamp(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    DateTime::<Utc>::from_utc(naive, Utc)
}
