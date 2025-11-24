//! 时间工具模块

use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::debug;

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
        let datetime = DateTime::parse_from_rfc3339(iso_string).map_err(|e| {
            crate::error::AuthError::TimeError(format!("Invalid ISO format: {}", e))
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
        let naive_datetime = NaiveDateTime::parse_from_str(time_str, format)
            .map_err(|e| crate::error::AuthError::TimeError(format!("Parse error: {}", e)))?;

        let datetime = Utc.from_utc_datetime(&naive_datetime);
        Ok(datetime.timestamp() as u64)
    }

    /// 计算两个时间戳之间的差值（毫秒）
    pub fn timestamp_diff_millis(start: u64, end: u64) -> i64 {
        (end as i64 - start as i64) * 1000
    }

    /// 计算两个时间戳之间的差值（秒）
    pub fn timestamp_diff_seconds(start: u64, end: u64) -> i64 {
        end as i64 - start as i64
    }

    /// 添加时间间隔
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

    /// 检查时间戳是否已过期
    pub fn is_timestamp_expired(timestamp: u64, current_time: u64) -> bool {
        timestamp <= current_time
    }

    /// 检查时间戳是否在指定时间内过期
    pub fn is_timestamp_expiring_soon(timestamp: u64, threshold_seconds: u64) -> bool {
        let current_time = Self::current_timestamp();
        let time_until_expiry = timestamp.saturating_sub(current_time);

        time_until_expiry <= threshold_seconds
    }

    /// 计算时间戳剩余有效时间
    pub fn time_until_expiry(timestamp: u64) -> Option<Duration> {
        let current_time = Self::current_timestamp();

        if timestamp > current_time {
            Some(Duration::from_secs(timestamp - current_time))
        } else {
            None
        }
    }

    /// 计算时间戳已过期时间
    pub fn time_since_expiry(timestamp: u64) -> Option<Duration> {
        let current_time = Self::current_timestamp();

        if timestamp <= current_time {
            Some(Duration::from_secs(current_time - timestamp))
        } else {
            None
        }
    }

    /// 获取今日开始时间戳
    pub fn today_start_timestamp() -> u64 {
        let now = Utc::now();
        let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
        Utc.from_utc_datetime(&today_start).timestamp() as u64
    }

    /// 获取今日结束时间戳
    pub fn today_end_timestamp() -> u64 {
        let now = Utc::now();
        let today_end = now.date_naive().and_hms_opt(23, 59, 59).unwrap();
        Utc.from_utc_datetime(&today_end).timestamp() as u64
    }

    /// 获取本周开始时间戳（周一）
    pub fn week_start_timestamp() -> u64 {
        let now = Utc::now();
        let weekday = now.weekday().num_days_from_monday();
        let week_start = now.date_naive() - chrono::Days::new(weekday as u64);
        let start_datetime = week_start.and_hms_opt(0, 0, 0).unwrap();
        Utc.from_utc_datetime(&start_datetime).timestamp() as u64
    }

    /// 获取本月开始时间戳
    pub fn month_start_timestamp() -> u64 {
        let now = Utc::now();
        let month_start = now.date_naive().with_day(1).unwrap();
        let start_datetime = month_start.and_hms_opt(0, 0, 0).unwrap();
        Utc.from_utc_datetime(&start_datetime).timestamp() as u64
    }

    /// 格式化持续时间为人类可读格式
    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let days = total_seconds / 86400;
        let hours = (total_seconds % 86400) / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        let mut parts = Vec::new();

        if days > 0 {
            parts.push(format!("{}天", days));
        }
        if hours > 0 {
            parts.push(format!("{}小时", hours));
        }
        if minutes > 0 {
            parts.push(format!("{}分钟", minutes));
        }
        if seconds > 0 || parts.is_empty() {
            parts.push(format!("{}秒", seconds));
        }

        parts.join(" ")
    }

    /// 解析人类可读的持续时间
    pub fn parse_duration(duration_str: &str) -> Result<Duration, crate::error::AuthError> {
        let duration_str = duration_str.trim().to_lowercase();

        if let Ok(seconds) = duration_str.parse::<u64>() {
            return Ok(Duration::from_secs(seconds));
        }

        // 解析格式如 "1h 30m", "45m", "2d 3h" 等
        let mut total_seconds = 0u64;
        let mut current_number = String::new();

        for c in duration_str.chars() {
            if c.is_ascii_digit() {
                current_number.push(c);
            } else if c.is_ascii_alphabetic() {
                if current_number.is_empty() {
                    return Err(crate::error::AuthError::TimeError(
                        "Invalid duration format".to_string(),
                    ));
                }

                let number = current_number.parse::<u64>().map_err(|_| {
                    crate::error::AuthError::TimeError("Invalid number in duration".to_string())
                })?;

                let seconds = match c {
                    's' => number,
                    'm' => number * 60,
                    'h' => number * 3600,
                    'd' => number * 86400,
                    'w' => number * 604800,
                    _ => {
                        return Err(crate::error::AuthError::TimeError(format!(
                            "Unsupported duration unit: {}",
                            c
                        )))
                    }
                };

                total_seconds += seconds;
                current_number.clear();
            } else if c.is_whitespace() {
                continue;
            } else {
                return Err(crate::error::AuthError::TimeError(
                    "Invalid character in duration".to_string(),
                ));
            }
        }

        if !current_number.is_empty() {
            return Err(crate::error::AuthError::TimeError(
                "Duration ends with number".to_string(),
            ));
        }

        Ok(Duration::from_secs(total_seconds))
    }

    /// 获取时间延迟（用于重试等场景）
    pub fn calculate_backoff_delay(
        attempt: u32,
        base_delay: Duration,
        max_delay: Duration,
    ) -> Duration {
        let delay = base_delay * 2_u32.pow(attempt.saturating_sub(1));
        std::cmp::min(delay, max_delay)
    }

    /// 获取抖动延迟（添加随机性避免雷群效应）
    pub fn jitter_delay(base_delay: Duration, jitter_factor: f64) -> Duration {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let jitter = rng.gen_range(0.0..=jitter_factor);
        let multiplier = 1.0 + jitter;

        let new_delay = Duration::from_millis((base_delay.as_millis() as f64 * multiplier) as u64);

        std::cmp::min(new_delay, base_delay * 2) // 避免过度延迟
    }

    /// 检查时间戳是否在指定范围内
    pub fn is_timestamp_in_range(timestamp: u64, start: u64, end: u64) -> bool {
        timestamp >= start && timestamp <= end
    }

    /// 获取时间窗口（用于滑动窗口算法等）
    pub fn get_time_window(timestamp: u64, window_size: Duration) -> u64 {
        timestamp / window_size.as_secs()
    }

    /// 对齐时间戳到指定间隔的边界
    pub fn align_timestamp(timestamp: u64, interval: Duration) -> u64 {
        (timestamp / interval.as_secs()) * interval.as_secs()
    }
}

/// 时间范围
#[derive(Debug, Clone)]
pub struct TimeRange {
    /// 开始时间戳
    pub start: u64,
    /// 结束时间戳
    pub end: u64,
}

impl TimeRange {
    /// 创建新的时间范围
    pub fn new(start: u64, end: u64) -> Result<Self, crate::error::AuthError> {
        if start > end {
            return Err(crate::error::AuthError::TimeError(
                "Start time cannot be after end time".to_string(),
            ));
        }

        Ok(Self { start, end })
    }

    /// 检查时间戳是否在范围内
    pub fn contains(&self, timestamp: u64) -> bool {
        timestamp >= self.start && timestamp <= self.end
    }

    /// 获取范围持续时间
    pub fn duration(&self) -> Duration {
        Duration::from_secs(self.end - self.start)
    }

    /// 检查范围是否重叠
    pub fn overlaps(&self, other: &TimeRange) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    /// 合并两个时间范围（如果重叠）
    pub fn merge(&self, other: &TimeRange) -> Option<TimeRange> {
        if !self.overlaps(other) {
            return None;
        }

        Some(TimeRange {
            start: std::cmp::min(self.start, other.start),
            end: std::cmp::max(self.end, other.end),
        })
    }

    /// 获取交集
    pub fn intersection(&self, other: &TimeRange) -> Option<TimeRange> {
        if !self.overlaps(other) {
            return None;
        }

        Some(TimeRange {
            start: std::cmp::max(self.start, other.start),
            end: std::cmp::min(self.end, other.end),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_timestamp() {
        let ts1 = TimeUtils::current_timestamp();
        std::thread::sleep(Duration::from_millis(10));
        let ts2 = TimeUtils::current_timestamp();

        assert!(ts2 > ts1);
        assert!(ts2 - ts1 >= 1); // 至少相差1秒
    }

    #[test]
    fn test_timestamp_conversion() {
        let timestamp = 1640995200; // 2022-01-01 00:00:00 UTC
        let iso_string = TimeUtils::timestamp_to_iso_string(timestamp);
        let parsed_timestamp = TimeUtils::iso_string_to_timestamp(&iso_string).unwrap();

        assert_eq!(timestamp, parsed_timestamp);
        assert!(iso_string.contains("2022-01-01"));
    }

    #[test]
    fn test_timestamp_diff() {
        let start = 1640995200;
        let end = 1640995300; // 100 seconds later

        assert_eq!(TimeUtils::timestamp_diff_seconds(start, end), 100);
        assert_eq!(TimeUtils::timestamp_diff_millis(start, end), 100000);
    }

    #[test]
    fn test_timestamp_expiry() {
        let now = TimeUtils::current_timestamp();
        let future = now + 3600; // 1 hour in future
        let past = now - 3600; // 1 hour in past

        assert!(!TimeUtils::is_timestamp_expired(future, now));
        assert!(TimeUtils::is_timestamp_expired(past, now));

        assert!(!TimeUtils::is_timestamp_expiring_soon(future, 300)); // Not expiring within 5 minutes
        assert!(TimeUtils::is_timestamp_expiring_soon(now + 100, 300)); // Expiring within 5 minutes
    }

    #[test]
    fn test_time_until_expiry() {
        let now = TimeUtils::current_timestamp();
        let future = now + 3600; // 1 hour in future

        let remaining = TimeUtils::time_until_expiry(future).unwrap();
        assert!(remaining.as_secs() >= 3590 && remaining.as_secs() <= 3610); // Allow some tolerance

        let none_remaining = TimeUtils::time_until_expiry(now - 1);
        assert!(none_remaining.is_none());
    }

    #[test]
    fn test_format_duration() {
        let duration = Duration::from_secs(3661); // 1 hour, 1 minute, 1 second
        let formatted = TimeUtils::format_duration(duration);
        assert_eq!(formatted, "1小时 1分钟 1秒");

        let short_duration = Duration::from_secs(30);
        let short_formatted = TimeUtils::format_duration(short_duration);
        assert_eq!(short_formatted, "30秒");
    }

    #[test]
    fn test_parse_duration() {
        let duration = TimeUtils::parse_duration("1h 30m").unwrap();
        assert_eq!(duration, Duration::from_secs(5400)); // 1 hour + 30 minutes

        let duration2 = TimeUtils::parse_duration("45m").unwrap();
        assert_eq!(duration2, Duration::from_secs(2700));

        let duration3 = TimeUtils::parse_duration("120").unwrap();
        assert_eq!(duration3, Duration::from_secs(120));
    }

    #[test]
    fn test_time_range() {
        let range1 = TimeRange::new(100, 200).unwrap();
        let range2 = TimeRange::new(150, 250).unwrap();
        let range3 = TimeRange::new(300, 400).unwrap();

        assert!(range1.contains(150));
        assert!(!range1.contains(250));

        assert!(range1.overlaps(&range2));
        assert!(!range1.overlaps(&range3));

        let merged = range1.merge(&range2).unwrap();
        assert_eq!(merged.start, 100);
        assert_eq!(merged.end, 250);

        let intersection = range1.intersection(&range2).unwrap();
        assert_eq!(intersection.start, 150);
        assert_eq!(intersection.end, 200);
    }

    #[test]
    fn test_backoff_delay() {
        let base = Duration::from_secs(1);
        let max = Duration::from_secs(10);

        let delay1 = TimeUtils::calculate_backoff_delay(1, base, max);
        assert_eq!(delay1, base);

        let delay2 = TimeUtils::calculate_backoff_delay(2, base, max);
        assert_eq!(delay2, Duration::from_secs(2));

        let delay4 = TimeUtils::calculate_backoff_delay(4, base, max);
        assert_eq!(delay4, Duration::from_secs(8));

        let delay5 = TimeUtils::calculate_backoff_delay(5, base, max);
        assert_eq!(delay5, max); // Should cap at max
    }

    #[test]
    fn test_jitter_delay() {
        let base = Duration::from_secs(10);
        let jittered = TimeUtils::jitter_delay(base, 0.5);

        // Should be between 10 and 20 seconds
        assert!(jittered >= base);
        assert!(jittered <= Duration::from_secs(20));
    }

    #[test]
    fn test_align_timestamp() {
        let interval = Duration::from_secs(300); // 5 minutes
        let timestamp = 1640995260; // Not aligned

        let aligned = TimeUtils::align_timestamp(timestamp, interval);
        assert_eq!(aligned, 1640995200); // Should be aligned to 5-minute boundary
    }
}
