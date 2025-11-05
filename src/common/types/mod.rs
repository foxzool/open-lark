//! 通用类型定义和类型别名
//!
//! 提供常用的类型别名、枚举和类型转换工具。

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// 用户ID类型别名
pub type UserId = String;
/// 租户ID类型别名
pub type TenantId = String;
/// 应用ID类型别名
pub type AppId = String;
/// 群组ID类型别名
pub type GroupId = String;
/// 部门ID类型别名
pub type DepartmentId = String;

/// 通用响应结果类型
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// 时间戳类型别名
pub type Timestamp = String;

/// 日期时间格式化器
pub mod datetime {
    use chrono::{DateTime, Local, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// 时间戳，支持ISO 8601格式
    #[derive(Debug, Clone)]
    pub struct TimestampWrapper(pub DateTime<Utc>);

    impl Serialize for TimestampWrapper {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let timestamp = self.0.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            serializer.serialize_str(&timestamp)
        }
    }

    impl<'de> Deserialize<'de> for TimestampWrapper {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let dt = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
            Ok(TimestampWrapper(dt.with_timezone(&Utc)))
        }
    }

    /// 本地时间包装器
    #[derive(Debug, Clone)]
    pub struct LocalDateTimeWrapper(pub DateTime<Local>);

    impl Serialize for LocalDateTimeWrapper {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let timestamp = self.0.format("%Y-%m-%d %H:%M:%S").to_string();
            serializer.serialize_str(&timestamp)
        }
    }

    impl<'de> Deserialize<'de> for LocalDateTimeWrapper {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let dt = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .map_err(serde::de::Error::custom)?;
            Ok(LocalDateTimeWrapper(dt.with_timezone(&Local)))
        }
    }
}

/// 可选字段处理器
pub mod optional {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// 可选字符串处理器，处理空字符串为None的情况
    pub struct OptionalString(pub Option<String>);

    impl Serialize for OptionalString {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match &self.0 {
                Some(s) if s.is_empty() => serializer.serialize_none(),
                Some(s) => serializer.serialize_some(s),
                None => serializer.serialize_none(),
            }
        }
    }

    impl<'de> Deserialize<'de> for OptionalString {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: Option<String> = Option::deserialize(deserializer)?;
            Ok(OptionalString(s.filter(|s| !s.is_empty())))
        }
    }

    impl From<Option<String>> for OptionalString {
        fn from(value: Option<String>) -> Self {
            OptionalString(value.filter(|s| !s.is_empty()))
        }
    }

    impl From<&str> for OptionalString {
        fn from(value: &str) -> Self {
            let s = value.trim();
            if s.is_empty() {
                OptionalString(None)
            } else {
                OptionalString(Some(s.to_string()))
            }
        }
    }

    impl From<String> for OptionalString {
        fn from(value: String) -> Self {
            let s = value.trim();
            if s.is_empty() {
                OptionalString(None)
            } else {
                OptionalString(Some(s.to_string()))
            }
        }
    }
}

/// 验证相关类型
pub mod validation {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// 邮箱地址验证器
    #[derive(Debug, Clone)]
    pub struct EmailAddress(pub String);

    impl EmailAddress {
        /// 验证邮箱格式
        pub fn is_valid(&self) -> bool {
            // 简单的邮箱格式验证，不依赖regex crate
            let email = &self.0;
            if email.is_empty() || email.len() > 254 {
                return false;
            }

            // 检查是否包含@符号
            let at_pos = match email.find('@') {
                Some(pos) => pos,
                None => return false,
            };

            // 检查@符号的位置
            if at_pos == 0 || at_pos == email.len() - 1 {
                return false;
            }

            // 检查@符号后是否有.
            let domain_part = &email[at_pos + 1..];
            domain_part.contains('.')
        }
    }

    impl Serialize for EmailAddress {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.0)
        }
    }

    impl<'de> Deserialize<'de> for EmailAddress {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: String = String::deserialize(deserializer)?;
            Ok(EmailAddress(s))
        }
    }

    impl From<String> for EmailAddress {
        fn from(value: String) -> Self {
            EmailAddress(value)
        }
    }

    /// 手机号码验证器
    #[derive(Debug, Clone)]
    pub struct PhoneNumber(pub String);

    impl PhoneNumber {
        /// 验证手机号格式（中国手机号）
        pub fn is_valid_cn(&self) -> bool {
            // 简单的中国手机号验证，不依赖regex crate
            let phone = &self.0;
            if phone.len() != 11 {
                return false;
            }

            // 检查是否以1开头
            if !phone.starts_with('1') {
                return false;
            }

            // 检查第二位是否在3-9范围内
            if let Some(second_char) = phone.chars().nth(1) {
                if !matches!(second_char, '3' | '4' | '5' | '6' | '7' | '8' | '9') {
                    return false;
                }
            } else {
                return false;
            }

            // 检查是否全为数字
            phone.chars().all(|c| c.is_ascii_digit())
        }

        /// 验证手机号格式（国际格式）
        pub fn is_valid_international(&self) -> bool {
            // 简单的国际手机号验证，不依赖regex crate
            let phone = &self.0;
            if phone.len() < 8 || phone.len() > 16 {
                return false;
            }

            // 检查是否以+开头
            if !phone.starts_with('+') {
                return false;
            }

            // 检查除+外是否全为数字
            phone.chars().skip(1).all(|c| c.is_ascii_digit())
        }
    }

    impl Serialize for PhoneNumber {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.0)
        }
    }

    impl<'de> Deserialize<'de> for PhoneNumber {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: String = String::deserialize(deserializer)?;
            Ok(PhoneNumber(s))
        }
    }

    impl From<String> for PhoneNumber {
        fn from(value: String) -> Self {
            PhoneNumber(value)
        }
    }
}

/// 响应状态码
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
pub enum StatusCode {
    /// 成功
    Success = 0,
    /// 参数错误
    InvalidParameter = 1,
    /// 未授权
    Unauthorized = 2,
    /// 禁止访问
    Forbidden = 3,
    /// 资源不存在
    NotFound = 4,
    /// 方法不允许
    MethodNotAllowed = 5,
    /// 请求过多
    TooManyRequests = 7,
    /// 内部服务器错误
    InternalServerError = 8,
    /// 服务不可用
    ServiceUnavailable = 9,
    /// 网关超时
    GatewayTimeout = 10,
}

impl StatusCode {
    /// 判断是否成功
    pub fn is_success(&self) -> bool {
        matches!(self, StatusCode::Success)
    }

    /// 判断是否为客户端错误
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            StatusCode::InvalidParameter
                | StatusCode::Unauthorized
                | StatusCode::Forbidden
                | StatusCode::NotFound
                | StatusCode::MethodNotAllowed
        )
    }

    /// 判断是否为服务器错误
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            StatusCode::InternalServerError
                | StatusCode::ServiceUnavailable
                | StatusCode::GatewayTimeout
        )
    }
}
