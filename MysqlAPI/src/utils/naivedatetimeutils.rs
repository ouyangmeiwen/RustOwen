use chrono::{Duration, Local, NaiveDateTime, Utc};

pub struct NaiveDateTimeUtils;

impl NaiveDateTimeUtils {
    /// 获取当前时间（NaiveDateTime），基于本地时区
    pub fn now_local() -> NaiveDateTime {
        Local::now().naive_local()
    }

    /// 获取当前时间（NaiveDateTime），基于 UTC 晚八个小时
    pub fn now_utc() -> NaiveDateTime {
        Utc::now().naive_utc()
    }

    /// 格式化 NaiveDateTime 为指定格式
    pub fn format_time(datetime: NaiveDateTime, fmt: &str) -> String {
        datetime.format(fmt).to_string()
    }

    /// 解析指定格式的字符串为 NaiveDateTime
    pub fn parse_from_str(
        datetime_str: &str,
        fmt: &str,
    ) -> Result<NaiveDateTime, chrono::ParseError> {
        NaiveDateTime::parse_from_str(datetime_str, fmt)
    }

    /// 获取 N 秒后的时间
    pub fn add_seconds(datetime: NaiveDateTime, seconds: i64) -> NaiveDateTime {
        datetime + Duration::seconds(seconds)
    }

    /// 获取 N 秒前的时间
    pub fn subtract_seconds(datetime: NaiveDateTime, seconds: i64) -> NaiveDateTime {
        datetime - Duration::seconds(seconds)
    }

    /// 计算两个 NaiveDateTime 的时间差（以秒为单位）
    pub fn difference_in_seconds(start: NaiveDateTime, end: NaiveDateTime) -> i64 {
        (end - start).num_seconds()
    }
}
