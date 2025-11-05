//! 通用工具函数
//!
//! 提供跨服务使用的工具函数和辅助方法。

use serde_json::Value;
use std::collections::HashMap;

/// JSON工具函数
pub mod json {
    use serde_json::Value;
    use std::collections::HashMap;

    /// 安全地获取JSON对象中的字符串值
    pub fn get_string(value: &Value, key: &str) -> Option<String> {
        value
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// 安全地获取JSON对象中的整数值
    pub fn get_i64(value: &Value, key: &str) -> Option<i64> {
        value.get(key).and_then(|v| v.as_i64())
    }

    /// 安全地获取JSON对象中的布尔值
    pub fn get_bool(value: &Value, key: &str) -> Option<bool> {
        value.get(key).and_then(|v| v.as_bool())
    }

    /// 安全地获取JSON对象中的数组
    pub fn get_array(value: &Value, key: &str) -> Option<Vec<Value>> {
        value.get(key).and_then(|v| v.as_array()).cloned()
    }

    /// 安全地获取JSON对象中的对象
    pub fn get_object(value: &Value, key: &str) -> Option<HashMap<String, Value>> {
        value
            .get(key)
            .and_then(|v| v.as_object())
            .map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }

    /// 将值转换为Option<T>
    pub fn to_option<T>(value: Value) -> Option<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        serde_json::from_value(value).ok()
    }
}

/// 字符串处理工具
pub mod string {
    /// 安全地截取字符串
    pub fn safe_truncate(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        }
    }

    /// 清理字符串中的空白字符
    pub fn clean_whitespace(s: &str) -> String {
        s.trim().to_string()
    }

    /// 检查字符串是否为空或只包含空白字符
    pub fn is_empty_or_whitespace(s: &str) -> bool {
        s.trim().is_empty()
    }

    /// 首字母大写
    pub fn capitalize_first(s: &str) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        if let Some(first) = chars.get_mut(0) {
            *first = first.to_uppercase().next().unwrap_or(*first);
        }
        chars.into_iter().collect()
    }

    /// 驺安全字符串处理
    pub fn safe_display(s: Option<&str>) -> String {
        s.map(|s| s.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    }
}

/// 验证工具
pub mod validation {
    /// 验证字符串是否符合正则表达式
    pub fn matches_regex(pattern: &str, text: &str) -> bool {
        regex::Regex::new(pattern)
            .map(|re| re.is_match(text))
            .unwrap_or(false)
    }

    /// 验证邮箱格式
    pub fn is_valid_email(email: &str) -> bool {
        matches_regex(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$", email)
    }

    /// 验证手机号格式（中国）
    pub fn is_valid_phone_cn(phone: &str) -> bool {
        matches_regex(r"^1[3-9]\d{9}$", phone)
    }

    /// 验证URL格式
    pub fn is_valid_url(url: &str) -> bool {
        matches_regex(r"^https?://[^\s/$.?#].[^\s]*$", url)
    }

    /// 验证UUID格式
    pub fn is_valid_uuid(uuid: &str) -> bool {
        matches_regex(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
            uuid,
        )
    }
}

/// 时间工具
pub mod time {
    use chrono::{DateTime, Local, TimeZone, Utc};

    /// 解析ISO 8601格式的时间戳
    pub fn parse_iso8601(timestamp: &str) -> Option<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(timestamp)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }

    /// 格式化时间为ISO 8601字符串
    pub fn format_iso8601(dt: &DateTime<Utc>) -> String {
        dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
    }

    /// 解析本地时间格式
    pub fn parse_local_time(timestamp: &str) -> Option<DateTime<Local>> {
        DateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S")
            .ok()
            .map(|dt| dt.with_timezone(&Local))
    }

    /// 格式化本地时间
    pub fn format_local_time(dt: &DateTime<Local>) -> String {
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// 获取当前时间戳
    pub fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }

    /// 将DateTime转换为时间戳字符串
    pub fn to_timestamp(dt: &DateTime<Utc>) -> String {
        dt.timestamp().to_string()
    }

    /// 从时间戳创建DateTime
    pub fn from_timestamp(ts: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(ts, 0).unwrap_or(Utc::now())
    }
}

/// 集合工具
pub mod combined {
    use super::string::safe_truncate;
    use super::validation::is_valid_email;

    /// 创建安全的显示字符串
    pub fn safe_display_string(value: Option<&str>, max_len: usize) -> String {
        let s = super::string::safe_display(value);
        safe_truncate(&s, max_len)
    }

    /// 验证并格式化邮箱
    pub fn validate_and_format_email(email: &str) -> Result<String, String> {
        if super::validation::is_valid_email(email) {
            Ok(email.to_lowercase())
        } else {
            Err("Invalid email format".to_string())
        }
    }

    /// 提取URL中的路径部分
    pub fn extract_path(url: &str) -> Option<String> {
        // 简单的路径提取，避免依赖url crate
        if let Some(path_start) = url.find("://") {
            let after_protocol = &url[path_start + 3..];
            if let Some(path_end) = after_protocol.find('/') {
                Some(after_protocol[path_end..].to_string())
            } else {
                None
            }
        } else if let Some(slash_pos) = url.find('/') {
            Some(url[slash_pos..].to_string())
        } else {
            None
        }
    }
}
