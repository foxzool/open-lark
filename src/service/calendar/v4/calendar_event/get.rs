//! # 获取日程事件
//!
//! 获取指定日程事件的详细信息。
//!
//! ## 功能描述
//!
//! 此接口用于获取指定日程事件的详细信息，包括事件标题、时间、
//! 地点、参与者、描述等所有相关信息。
//!
//! ## API端点
//!
//! GET `/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}`
//!
//! ## 权限要求
//!
//! 需要 `calendar:calendar:readonly` 权限。

use crate::service::calendar::v4::models::CalendarEvent;
use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取日程事件请求
///
/// 用于获取指定日程事件的详细信息的请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCalendarEventRequest {
    /// 日历ID
    pub calendar_id: String,
    /// 日程ID
    pub event_id: String,
    /// 用户ID类型
    ///
    /// 可选值：
    /// - `open_id`：用户的开放应用ID
    /// - `user_id`：用户的用户ID
    /// - `union_id`：用户的联合ID
    ///
    /// 默认值：`open_id`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetCalendarEventRequest {
    /// 创建获取日程事件请求
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    /// - `event_id`: 日程ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::calendar::v4::calendar_event::GetCalendarEventRequest;
    ///
    /// let request = GetCalendarEventRequest::new("calendar_123", "event_456");
    /// ```
    pub fn new(calendar_id: impl Into<String>, event_id: impl Into<String>) -> Self {
        Self {
            calendar_id: calendar_id.into(),
            event_id: event_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::GetCalendarEventRequest;
    /// let mut request = GetCalendarEventRequest::new("calendar_123", "event_456");
    /// request.set_user_id_type("user_id");
    /// ```
    pub fn set_user_id_type(&mut self, user_id_type: impl Into<String>) -> &mut Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    ///
    /// # 返回
    /// - `Ok(())`: 验证通过
    /// - `Err(String)`: 验证失败，返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.calendar_id.trim().is_empty() {
            return Err("日历ID不能为空".to_string());
        }

        if self.event_id.trim().is_empty() {
            return Err("日程ID不能为空".to_string());
        }

        if let Some(user_id_type) = &self.user_id_type {
            if user_id_type.trim().is_empty() {
                return Err("用户ID类型不能为空".to_string());
            }
        }

        Ok(())
    }
}

/// 获取日程事件响应
///
/// 包含指定日程事件的详细信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCalendarEventResponse {
    /// 日程事件信息
    pub event: Option<CalendarEvent>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
}

impl Default for GetCalendarEventResponse {
    fn default() -> Self {
        Self {
            event: None,
            has_more: None,
            page_token: None,
        }
    }
}

impl ApiResponseTrait for GetCalendarEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取日程事件构建器
///
/// 提供流式API来构建获取日程事件的请求。
#[derive(Debug, Clone)]
pub struct GetCalendarEventBuilder {
    request: GetCalendarEventRequest,
}

impl GetCalendarEventBuilder {
    /// 创建获取日程事件构建器
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    /// - `event_id`: 日程ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::calendar::v4::calendar_event::GetCalendarEventBuilder;
    ///
    /// let builder = GetCalendarEventBuilder::new("calendar_123", "event_456");
    /// ```
    pub fn new(calendar_id: impl Into<String>, event_id: impl Into<String>) -> Self {
        Self {
            request: GetCalendarEventRequest::new(calendar_id, event_id),
        }
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::GetCalendarEventBuilder;
    /// let builder = GetCalendarEventBuilder::new("calendar_123", "event_456")
    ///     .user_id_type("user_id");
    /// ```
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.set_user_id_type(user_id_type);
        self
    }

    /// 构建请求对象
    ///
    /// # 返回
    /// - `Ok(GetCalendarEventRequest)`: 构建成功的请求对象
    /// - `Err(String)`: 构建失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::GetCalendarEventBuilder;
    /// let request = GetCalendarEventBuilder::new("calendar_123", "event_456")
    ///     .user_id_type("open_id")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<GetCalendarEventRequest, String> {
        self.request.validate()?;
        Ok(self.request)
    }

    /// 执行获取日程事件请求
    ///
    /// # 参数
    /// - `service`: 日程事件服务实例
    ///
    /// # 返回
    /// - `Ok(GetCalendarEventResponse)`: 获取成功，返回日程事件信息
    /// - `Err(SDKError)`: 获取失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::GetCalendarEventBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let response = GetCalendarEventBuilder::new("calendar_123", "event_456")
    ///     .user_id_type("open_id")
    ///     .execute(&client.calendar.v4.calendar_event)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute(
        self,
        service: &crate::service::calendar::v4::calendar_event::CalendarEventService,
    ) -> SDKResult<GetCalendarEventResponse> {
        let request = self
            .build()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        service.get(&request).await
    }
}

impl Default for GetCalendarEventBuilder {
    fn default() -> Self {
        Self {
            request: GetCalendarEventRequest {
                calendar_id: String::new(),
                event_id: String::new(),
                user_id_type: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calendar_event_request_creation() {
        let request = GetCalendarEventRequest::new("calendar_123", "event_456");
        assert_eq!(request.calendar_id, "calendar_123");
        assert_eq!(request.event_id, "event_456");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_set_user_id_type() {
        let mut request = GetCalendarEventRequest::new("calendar_123", "event_456");
        request.set_user_id_type("user_id");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_request_validation_success() {
        let request = GetCalendarEventRequest::new("calendar_123", "event_456");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_request_validation_empty_calendar_id() {
        let request = GetCalendarEventRequest::new("", "event_456");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_request_validation_empty_event_id() {
        let request = GetCalendarEventRequest::new("calendar_123", "");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日程ID不能为空");
    }

    #[test]
    fn test_request_validation_empty_user_id_type() {
        let mut request = GetCalendarEventRequest::new("calendar_123", "event_456");
        request.set_user_id_type("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_get_calendar_event_builder_creation() {
        let builder = GetCalendarEventBuilder::new("calendar_123", "event_456");
        assert_eq!(builder.request.calendar_id, "calendar_123");
        assert_eq!(builder.request.event_id, "event_456");
        assert!(builder.request.user_id_type.is_none());
    }

    #[test]
    fn test_builder_user_id_type() {
        let builder =
            GetCalendarEventBuilder::new("calendar_123", "event_456").user_id_type("open_id");
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_builder_build_success() {
        let result = GetCalendarEventBuilder::new("calendar_123", "event_456")
            .user_id_type("open_id")
            .build();
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.calendar_id, "calendar_123");
        assert_eq!(request.event_id, "event_456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_builder_build_failure_empty_calendar_id() {
        let result = GetCalendarEventBuilder::new("", "event_456").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_builder_build_failure_empty_event_id() {
        let result = GetCalendarEventBuilder::new("calendar_123", "").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日程ID不能为空");
    }

    #[test]
    fn test_builder_default() {
        let builder = GetCalendarEventBuilder::default();
        assert_eq!(builder.request.calendar_id, "");
        assert_eq!(builder.request.event_id, "");
        assert!(builder.request.user_id_type.is_none());
    }

    #[test]
    fn test_request_with_whitespace_calendar_id() {
        let request = GetCalendarEventRequest::new("   ", "event_456");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_request_with_whitespace_event_id() {
        let request = GetCalendarEventRequest::new("calendar_123", "   ");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日程ID不能为空");
    }

    #[test]
    fn test_request_with_whitespace_user_id_type() {
        let mut request = GetCalendarEventRequest::new("calendar_123", "event_456");
        request.set_user_id_type("   ");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_serialization_request() {
        let request = GetCalendarEventRequest {
            calendar_id: "calendar_123".to_string(),
            event_id: "event_456".to_string(),
            user_id_type: Some("open_id".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("calendar_123"));
        assert!(json.contains("event_456"));
        assert!(json.contains("open_id"));
    }

    #[test]
    fn test_serialization_request_without_optional_fields() {
        let request = GetCalendarEventRequest {
            calendar_id: "calendar_123".to_string(),
            event_id: "event_456".to_string(),
            user_id_type: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("calendar_123"));
        assert!(json.contains("event_456"));
        assert!(!json.contains("user_id_type"));
    }

    #[test]
    fn test_deserialization_response() {
        let json = r#"
        {
            "event": {
                "event_id": "event_456",
                "summary": "Test Event",
                "description": "Test Description"
            },
            "has_more": false,
            "page_token": "next_page_token"
        }
        "#;
        let response: GetCalendarEventResponse = serde_json::from_str(json).unwrap();
        assert!(response.event.is_some());
        assert_eq!(
            response.event.unwrap().event_id,
            Some("event_456".to_string())
        );
        assert_eq!(response.has_more, Some(false));
        assert_eq!(response.page_token, Some("next_page_token".to_string()));
    }
}
