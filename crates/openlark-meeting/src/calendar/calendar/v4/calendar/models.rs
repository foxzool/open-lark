use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 获取日历信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetCalendarResponse {
    /// 日历详情。
    pub calendar: Calendar,
}

/// 创建日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCalendarResponse {
    /// 新创建的日历详情。
    pub calendar: Calendar,
}

/// 日历信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Calendar {
    /// 日历 ID。
    pub calendar_id: String,
    /// 日历标题。
    pub summary: String,
    /// 日历描述。
    pub description: Option<String>,
    /// 日历颜色。
    pub color: Option<String>,
    /// 当前访问权限。
    pub permissions: Option<CalendarPermissions>,
    /// 是否为主日历。
    pub primary: Option<bool>,
    /// 日历类型。
    pub calendar_type: Option<String>,
}

/// 日历权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarPermissions {
    /// 是否可读。
    pub is_readable: Option<bool>,
    /// 是否可写。
    pub is_writable: Option<bool>,
}

impl ApiResponseTrait for GetCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(
        original: &T,
    ) {
        let json = serde_json::to_string(original).expect("序列化失败");
        let deserialized: T = serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(original, &deserialized, "roundtrip 后数据不一致");
    }

    #[test]
    fn test_calendar_permissions_serialization() {
        let permissions = CalendarPermissions {
            is_readable: Some(true),
            is_writable: Some(false),
        };
        test_roundtrip(&permissions);
    }

    #[test]
    fn test_calendar_serialization() {
        let calendar = Calendar {
            calendar_id: "cal123".to_string(),
            summary: "工作日历".to_string(),
            description: Some("团队工作日历".to_string()),
            color: Some("#1890ff".to_string()),
            permissions: Some(CalendarPermissions {
                is_readable: Some(true),
                is_writable: Some(true),
            }),
            primary: Some(true),
            calendar_type: Some("shared".to_string()),
        };
        test_roundtrip(&calendar);
    }

    #[test]
    fn test_get_calendar_response_serialization() {
        let response = GetCalendarResponse {
            calendar: Calendar {
                calendar_id: "cal123".to_string(),
                summary: "工作日历".to_string(),
                description: None,
                color: None,
                permissions: None,
                primary: None,
                calendar_type: None,
            },
        };
        test_roundtrip(&response);
    }

    #[test]
    fn test_create_calendar_response_serialization() {
        let response = CreateCalendarResponse {
            calendar: Calendar {
                calendar_id: "cal456".to_string(),
                summary: "项目日历".to_string(),
                description: Some("项目相关日程".to_string()),
                color: Some("#52c41a".to_string()),
                permissions: None,
                primary: Some(false),
                calendar_type: None,
            },
        };
        test_roundtrip(&response);
    }
}
