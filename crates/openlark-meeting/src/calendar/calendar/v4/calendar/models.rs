use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 获取日历信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetCalendarResponse {
    pub calendar: Calendar,
}

/// 创建日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCalendarResponse {
    pub calendar: Calendar,
}

/// 日历信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Calendar {
    pub calendar_id: String,
    pub summary: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub permissions: Option<CalendarPermissions>,
    pub primary: Option<bool>,
    pub calendar_type: Option<String>,
}

/// 日历权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarPermissions {
    pub is_readable: Option<bool>,
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
