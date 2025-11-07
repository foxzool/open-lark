//! # 删除日程事件
//!
//! 删除指定的日程事件。删除后不可恢复。
//!
//! ## 功能描述
//!
//! 此接口用于删除指定的日程事件，删除后无法恢复。请谨慎使用此功能。
//!
//! ## API端点
//!
//! DELETE `/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}`
//!
//! ## 权限要求
//!
//! 需要 `calendar:calendar` 权限。
//!
//! ## 注意事项
//!
//! - 删除操作不可逆，请确认后再执行
//! - 删除后该事件的所有相关信息将被清除
//! - 如果事件有参与者，他们将收到删除通知

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    http::Transport,
    ApiRequest, SDKResult,
};
use crate::service::calendar::v4::models::CalendarEvent;
use serde::{Deserialize, Serialize};

/// 删除日程事件请求
///
/// 用于删除指定日程事件的请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCalendarEventRequest {
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

impl DeleteCalendarEventRequest {
    /// 创建删除日程事件请求
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    /// - `event_id`: 日程ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::calendar::v4::calendar_event::DeleteCalendarEventRequest;
    ///
    /// let request = DeleteCalendarEventRequest::new("calendar_123", "event_456");
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
    /// # use open_lark::service::calendar::v4::calendar_event::DeleteCalendarEventRequest;
    /// let mut request = DeleteCalendarEventRequest::new("calendar_123", "event_456");
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

/// 删除日程事件响应
///
/// 包含删除操作的结果信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCalendarEventResponse {
    /// 是否删除成功
    pub success: Option<bool>,
    /// 删除的日程事件信息（删除前）
    pub event: Option<CalendarEvent>,
    /// 操作结果消息
    pub message: Option<String>,
}

impl Default for DeleteCalendarEventResponse {
    fn default() -> Self {
        Self {
            success: None,
            event: None,
            message: None,
        }
    }
}

impl ApiResponseTrait for DeleteCalendarEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除日程事件构建器
///
/// 提供流式API来构建删除日程事件的请求。
#[derive(Debug, Clone)]
pub struct DeleteCalendarEventBuilder {
    request: DeleteCalendarEventRequest,
}

impl DeleteCalendarEventBuilder {
    /// 创建删除日程事件构建器
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    /// - `event_id`: 日程ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::calendar::v4::calendar_event::DeleteCalendarEventBuilder;
    ///
    /// let builder = DeleteCalendarEventBuilder::new("calendar_123", "event_456");
    /// ```
    pub fn new(calendar_id: impl Into<String>, event_id: impl Into<String>) -> Self {
        Self {
            request: DeleteCalendarEventRequest::new(calendar_id, event_id),
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
    /// # use open_lark::service::calendar::v4::calendar_event::DeleteCalendarEventBuilder;
    /// let builder = DeleteCalendarEventBuilder::new("calendar_123", "event_456")
    ///     .user_id_type("user_id");
    /// ```
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.set_user_id_type(user_id_type);
        self
    }

    /// 构建请求对象
    ///
    /// # 返回
    /// - `Ok(DeleteCalendarEventRequest)`: 构建成功的请求对象
    /// - `Err(String)`: 构建失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::DeleteCalendarEventBuilder;
    /// let request = DeleteCalendarEventBuilder::new("calendar_123", "event_456")
    ///     .user_id_type("open_id")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<DeleteCalendarEventRequest, String> {
        self.request.validate()?;
        Ok(self.request)
    }

    /// 执行删除日程事件请求
    ///
    /// # 参数
    /// - `service`: 日程事件服务实例
    ///
    /// # 返回
    /// - `Ok(DeleteCalendarEventResponse)`: 删除成功，返回操作结果
    /// - `Err(SDKError)`: 删除失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::DeleteCalendarEventBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let response = DeleteCalendarEventBuilder::new("calendar_123", "event_456")
    ///     .user_id_type("open_id")
    ///     .execute(&client.calendar.v4.calendar_event)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute(
        self,
        service: &crate::service::calendar::v4::calendar_event::CalendarEventService,
    ) -> SDKResult<DeleteCalendarEventResponse> {
        let request = self
            .build()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        service.delete(&request).await
    }
}

impl Default for DeleteCalendarEventBuilder {
    fn default() -> Self {
        Self {
            request: DeleteCalendarEventRequest {
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
    fn test_delete_calendar_event_request_creation() {
        let request = DeleteCalendarEventRequest::new("calendar_123", "event_456");
        assert_eq!(request.calendar_id, "calendar_123");
        assert_eq!(request.event_id, "event_456");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_set_user_id_type() {
        let mut request = DeleteCalendarEventRequest::new("calendar_123", "event_456");
        request.set_user_id_type("user_id");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_request_validation_success() {
        let request = DeleteCalendarEventRequest::new("calendar_123", "event_456");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_request_validation_empty_calendar_id() {
        let request = DeleteCalendarEventRequest::new("", "event_456");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_request_validation_empty_event_id() {
        let request = DeleteCalendarEventRequest::new("calendar_123", "");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日程ID不能为空");
    }

    #[test]
    fn test_request_validation_empty_user_id_type() {
        let mut request = DeleteCalendarEventRequest::new("calendar_123", "event_456");
        request.set_user_id_type("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_delete_calendar_event_builder_creation() {
        let builder = DeleteCalendarEventBuilder::new("calendar_123", "event_456");
        assert_eq!(builder.request.calendar_id, "calendar_123");
        assert_eq!(builder.request.event_id, "event_456");
        assert!(builder.request.user_id_type.is_none());
    }

    #[test]
    fn test_builder_user_id_type() {
        let builder =
            DeleteCalendarEventBuilder::new("calendar_123", "event_456").user_id_type("open_id");
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_builder_build_success() {
        let result = DeleteCalendarEventBuilder::new("calendar_123", "event_456")
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
        let result = DeleteCalendarEventBuilder::new("", "event_456").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_builder_build_failure_empty_event_id() {
        let result = DeleteCalendarEventBuilder::new("calendar_123", "").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日程ID不能为空");
    }

    #[test]
    fn test_builder_default() {
        let builder = DeleteCalendarEventBuilder::default();
        assert_eq!(builder.request.calendar_id, "");
        assert_eq!(builder.request.event_id, "");
        assert!(builder.request.user_id_type.is_none());
    }

    #[test]
    fn test_request_with_whitespace_calendar_id() {
        let request = DeleteCalendarEventRequest::new("   ", "event_456");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_request_with_whitespace_event_id() {
        let request = DeleteCalendarEventRequest::new("calendar_123", "   ");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日程ID不能为空");
    }

    #[test]
    fn test_request_with_whitespace_user_id_type() {
        let mut request = DeleteCalendarEventRequest::new("calendar_123", "event_456");
        request.set_user_id_type("   ");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_serialization_request() {
        let request = DeleteCalendarEventRequest {
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
        let request = DeleteCalendarEventRequest {
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
            "success": true,
            "event": {
                "event_id": "event_456",
                "summary": "Deleted Event",
                "description": "This event has been deleted"
            },
            "message": "删除成功"
        }
        "#;
        let response: DeleteCalendarEventResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.success, Some(true));
        assert!(response.event.is_some());
        assert_eq!(
            response.event.unwrap().event_id,
            Some("event_456".to_string())
        );
        assert_eq!(response.message, Some("删除成功".to_string()));
    }

    #[test]
    fn test_default_response() {
        let response = DeleteCalendarEventResponse::default();
        assert_eq!(response.success, None);
        assert!(response.event.is_none());
        assert_eq!(response.message, None);
    }

    #[test]
    fn test_fluent_api_chain() {
        let result = DeleteCalendarEventBuilder::new("calendar_123", "event_456")
            .user_id_type("user_id")
            .build();
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.calendar_id, "calendar_123");
        assert_eq!(request.event_id, "event_456");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_validation_edge_cases() {
        // Test with very long IDs
        let long_id = "a".repeat(1000);
        let request = DeleteCalendarEventRequest::new(&long_id, &long_id);
        assert!(request.validate().is_ok());

        // Test with Unicode characters
        let unicode_request = DeleteCalendarEventRequest::new("日历_123", "事件_456");
        assert!(unicode_request.validate().is_ok());
    }

    #[test]
    fn test_builder_fluent_with_all_options() {
        let builder =
            DeleteCalendarEventBuilder::new("calendar_123", "event_456").user_id_type("union_id");

        assert_eq!(builder.request.calendar_id, "calendar_123");
        assert_eq!(builder.request.event_id, "event_456");
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));

        let request = builder.build().unwrap();
        assert!(request.validate().is_ok());
    }
}
