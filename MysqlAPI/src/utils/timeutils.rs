use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TimeUtils;

impl TimeUtils {
    /// 获取系统当前时间（本地时区）
    pub fn now_local() -> DateTime<Local> {
        Local::now()
    }

    /// 获取系统当前时间（UTC）
    pub fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }

    /// 获取当前时间的时间戳（秒级）
    pub fn current_timestamp() -> i64 {
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_epoch.as_secs() as i64
    }

    /// 将时间戳（秒级）转换为本地时间
    pub fn timestamp_to_local(timestamp: i64) -> DateTime<Local> {
        let naive_datetime = NaiveDateTime::from_timestamp(timestamp, 0);
        Local.from_utc_datetime(&naive_datetime)
    }

    pub fn format_time<T>(datetime: DateTime<T>, fmt: &str) -> String
    where
        T: TimeZone,
        T::Offset: std::fmt::Display,
    {
        datetime.format(fmt).to_string()
    }
    /// 计算两个时间点之间的差距
    pub fn duration_between<T: chrono::TimeZone>(start: DateTime<T>, end: DateTime<T>) -> Duration {
        end.signed_duration_since(start)
    }
}
