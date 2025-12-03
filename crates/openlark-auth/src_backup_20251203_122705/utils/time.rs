//! 时间工具模块

use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// 时间工具
pub struct TimeUtils;

impl TimeUtils {
    /// 获取当前时间戳（秒）
    pub fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }

    /// 获取当前时间戳（毫秒）
    pub fn current_timestamp_millis() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }

    /// 时间戳转UTC时间
    pub fn timestamp_to_utc(timestamp: u64) -> DateTime<Utc> {
        Utc.timestamp_opt(timestamp as i64, 0)
            .single()
            .expect("Invalid timestamp")
    }

    /// UTC时间转时间戳
    pub fn utc_to_timestamp(datetime: &DateTime<Utc>) -> u64 {
        datetime.timestamp() as u64
    }

    /// 时间戳转字符串（ISO 8601格式）
    pub fn timestamp_to_iso_string(timestamp: u64) -> String {
        let datetime = Self::timestamp_to_utc(timestamp);
        datetime.to_rfc3339()
    }

    /// ISO 8601字符串转时间戳
    pub fn iso_string_to_timestamp(iso_string: &str) -> Result<u64, crate::error::AuthError> {
        let datetime = DateTime::parse_from_rfc3339(iso_string).map_err(|_| {
            crate::error::AuthErrorBuilder::validation_error("time", "Invalid ISO format", None::<String>)
        })?;

        Ok(datetime.timestamp() as u64)
    }

    /// 格式化时间戳为可读字符串
    pub fn format_timestamp(timestamp: u64, format: &str) -> String {
        let datetime = Self::timestamp_to_utc(timestamp);
        datetime.format(format).to_string()
    }

    /// 解析格式化的时间字符串
    pub fn parse_formatted_time(
        time_str: &str,
        format: &str,
    ) -> Result<u64, crate::error::AuthError> {
        let naive_datetime = NaiveDateTime::parse_from_str(time_str, format).map_err(|_| {
            crate::error::AuthErrorBuilder::validation_error("time", "Invalid time format", None::<String>)
        })?;

        let datetime = Utc.from_utc_datetime(&naive_datetime);
        Ok(datetime.timestamp() as u64)
    }

    /// 加时间间隔
    pub fn add_duration(timestamp: u64, duration: Duration) -> u64 {
        timestamp + duration.as_secs()
    }

    /// 减去时间间隔
    pub fn subtract_duration(timestamp: u64, duration: Duration) -> u64 {
        timestamp.saturating_sub(duration.as_secs())
    }

    /// 检查时间戳是否在过去
    pub fn is_timestamp_in_past(timestamp: u64) -> bool {
        timestamp < Self::current_timestamp()
    }

    /// 检查时间戳是否在未来
    pub fn is_timestamp_in_future(timestamp: u64) -> bool {
        timestamp > Self::current_timestamp()
    }

    /// 计算两个时间戳之间的差值（绝对值）
    pub fn timestamp_distance(timestamp1: u64, timestamp2: u64) -> Duration {
        Duration::from_secs(timestamp1.abs_diff(timestamp2))
    }

    /// 检查时间戳是否在指定范围内
    pub fn is_timestamp_in_range(
        timestamp: u64,
        start: u64,
        end: u64,
    ) -> bool {
        timestamp >= start && timestamp <= end
    }

    /// 获取时间戳的年份
    pub fn get_year(timestamp: u64) -> i32 {
        let datetime = Self::timestamp_to_utc(timestamp);
        datetime.year()
    }

    /// 获取时间戳的月份
    pub fn get_month(timestamp: u64) -> u32 {
        let datetime = Self::timestamp_to_utc(timestamp);
        datetime.month()
    }

    /// 获取时间戳的日期
    pub fn get_day(timestamp: u64) -> u32 {
        let datetime = Self::timestamp_to_utc(timestamp);
        datetime.day()
    }

    /// 计算时间戳到当前时间的持续时间
    pub fn time_since(timestamp: u64) -> Duration {
        let current = Self::current_timestamp();
        if timestamp <= current {
            Duration::from_secs(current - timestamp)
        } else {
            Duration::from_secs(timestamp - current)
        }
    }

    /// 计算当前时间到时间戳的持续时间
    pub fn time_until(timestamp: u64) -> Option<Duration> {
        let current = Self::current_timestamp();
        if timestamp > current {
            Some(Duration::from_secs(timestamp - current))
        } else {
            None
        }
    }

    /// 解析持续时间字符串（如 "1h 30m"）
    pub fn parse_duration(duration_str: &str) -> Result<Duration, crate::error::AuthError> {
        let mut total_seconds = 0u64;
        let mut current_number = String::new();

        for ch in duration_str.chars() {
            if ch.is_ascii_digit() {
                current_number.push(ch);
            } else if ch.is_ascii_whitespace() {
                continue;
            } else {
                if current_number.is_empty() {
                    return Err(crate::error::AuthErrorBuilder::validation_error(
                        "time",
                        "Invalid duration format: missing number",
                        None::<String>
                    ));
                }

                let number = current_number.parse::<u64>().map_err(|_| {
                    crate::error::AuthErrorBuilder::validation_error("time", "Invalid number in duration", None::<String>)
                })?;

                match ch {
                    's' => total_seconds += number,
                    'm' => total_seconds += number * 60,
                    'h' => total_seconds += number * 3600,
                    'd' => total_seconds += number * 86400,
                    _ => {
                        return Err(crate::error::AuthErrorBuilder::validation_error(
                            "time",
                            format!("Unsupported duration unit: {}", ch),
                            None::<String>
                        ));
                    }
                }

                current_number.clear();
            }
        }

        if !current_number.is_empty() {
            return Err(crate::error::AuthErrorBuilder::validation_error(
                "time",
                "Invalid duration format: trailing number",
                None::<String>
            ));
        }

        Ok(Duration::from_secs(total_seconds))
    }

    /// 格式化持续时间
    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let days = total_seconds / 86400;
        let hours = (total_seconds % 86400) / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        let mut result = String::new();

        if days > 0 {
            result.push_str(&format!("{}d ", days));
        }
        if hours > 0 {
            result.push_str(&format!("{}h ", hours));
        }
        if minutes > 0 {
            result.push_str(&format!("{}m ", minutes));
        }
        if seconds > 0 || result.is_empty() {
            result.push_str(&format!("{}s", seconds));
        }

        result.trim().to_string()
    }

    /// 检查时间戳是否为同一天
    pub fn is_same_day(timestamp1: u64, timestamp2: u64) -> bool {
        let dt1 = Self::timestamp_to_utc(timestamp1);
        let dt2 = Self::timestamp_to_utc(timestamp2);
        dt1.date_naive() == dt2.date_naive()
    }

    /// 获取当天开始的时间戳
    pub fn start_of_day(timestamp: u64) -> u64 {
        let datetime = Self::timestamp_to_utc(timestamp);
        let start_of_day = datetime.date_naive().and_hms_opt(0, 0, 0)
            .expect("Invalid time");
        let utc_start = Utc.from_utc_datetime(&start_of_day);
        utc_start.timestamp() as u64
    }

    /// 获取当天结束的时间戳
    pub fn end_of_day(timestamp: u64) -> u64 {
        let datetime = Self::timestamp_to_utc(timestamp);
        let end_of_day = datetime.date_naive().and_hms_opt(23, 59, 59)
            .expect("Invalid time");
        let utc_end = Utc.from_utc_datetime(&end_of_day);
        utc_end.timestamp() as u64
    }

    /// 验证时间戳是否合理（在合理范围内）
    pub fn is_valid_timestamp(timestamp: u64) -> bool {
        // 检查是否在2000年到2100年之间
        const MIN_TIMESTAMP: u64 = 946684800; // 2000-01-01 00:00:00 UTC
        const MAX_TIMESTAMP: u64 = 4102444800; // 2100-01-01 00:00:00 UTC

        timestamp >= MIN_TIMESTAMP && timestamp <= MAX_TIMESTAMP
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_timestamp() {
        let timestamp = TimeUtils::current_timestamp();
        assert!(timestamp > 0);
        assert!(TimeUtils::is_valid_timestamp(timestamp));
    }

    #[test]
    fn test_timestamp_conversion() {
        let timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
        let iso_string = TimeUtils::timestamp_to_iso_string(timestamp);
        assert_eq!(iso_string, "2021-01-01T00:00:00+00:00");

        let parsed_timestamp = TimeUtils::iso_string_to_timestamp(&iso_string).unwrap();
        assert_eq!(parsed_timestamp, timestamp);
    }

    #[test]
    fn test_duration_parsing() {
        let duration = TimeUtils::parse_duration("1h 30m").unwrap();
        assert_eq!(duration.as_secs(), 5400); // 1 hour + 30 minutes

        let duration = TimeUtils::parse_duration("2d 3h 4m 5s").unwrap();
        assert_eq!(duration.as_secs(), 183845); // 2 days + 3 hours + 4 minutes + 5 seconds
    }

    #[test]
    fn test_duration_formatting() {
        let duration = Duration::from_secs(3665); // 1h 1m 5s
        let formatted = TimeUtils::format_duration(duration);
        assert_eq!(formatted, "1h 1m 5s");
    }

    #[test]
    fn test_time_utilities() {
        let timestamp = TimeUtils::current_timestamp();

        // Test same day
        assert!(TimeUtils::is_same_day(timestamp, timestamp));

        // Test start/end of day
        let start = TimeUtils::start_of_day(timestamp);
        let end = TimeUtils::end_of_day(timestamp);
        assert!(start <= timestamp);
        assert!(timestamp <= end);
    }
}