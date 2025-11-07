//! # 创建日程事件
//!
//! 在指定日历中创建一个日程事件。
//!
//! ## 功能描述
//!
//! 此接口用于在用户日历中创建新的日程事件，支持设置事件标题、时间、地点、
//! 参与者、描述等信息。
//!
//! ## API端点
//!
//! POST `/open-apis/calendar/v4/calendars/{calendar_id}/events`
//!
//! ## 权限要求
//!
//! 需要 `calendar:calendar` 权限。

use serde::{Deserialize, Serialize};
use crate::core::{http::Transport, SDKResult, ApiRequest, api_resp::{ApiResponseTrait, ResponseFormat}};
use crate::service::calendar::v4::models::{CalendarEvent, TimeInfo, Reminder, EventAttendee, MeetingRoom, Location, EventStatus};

/// 创建日程事件请求
///
/// 用于在指定日历中创建新的事件的请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCalendarEventRequest {
    /// 日历ID
    pub calendar_id: String,
    /// 日程标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 日程描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeInfo>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeInfo>,
    /// 是否全天日程
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
    /// 重复规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    /// 提醒设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<Reminder>>,
    /// 参与人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<EventAttendee>>,
    /// 会议室
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_rooms: Option<Vec<MeetingRoom>>,
    /// 位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// 日程颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
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

impl CreateCalendarEventRequest {
    /// 创建创建日程事件请求
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::calendar::v4::calendar_event::CreateCalendarEventRequest;
    ///
    /// let request = CreateCalendarEventRequest::new("calendar_123");
    /// ```
    pub fn new(calendar_id: impl Into<String>) -> Self {
        Self {
            calendar_id: calendar_id.into(),
            summary: None,
            description: None,
            start_time: None,
            end_time: None,
            is_all_day: None,
            recurrence: None,
            reminders: None,
            attendees: None,
            meeting_rooms: None,
            location: None,
            color: None,
            user_id_type: None,
        }
    }

    /// 设置日程标题
    ///
    /// # 参数
    /// - `summary`: 日程标题
    pub fn set_summary(&mut self, summary: impl Into<String>) -> &mut Self {
        self.summary = Some(summary.into());
        self
    }

    /// 设置日程描述
    ///
    /// # 参数
    /// - `description`: 日程描述
    pub fn set_description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    /// 设置开始时间
    ///
    /// # 参数
    /// - `start_time`: 开始时间
    pub fn set_start_time(&mut self, start_time: TimeInfo) -> &mut Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    ///
    /// # 参数
    /// - `end_time`: 结束时间
    pub fn set_end_time(&mut self, end_time: TimeInfo) -> &mut Self {
        self.end_time = Some(end_time);
        self
    }

    /// 设置是否全天日程
    ///
    /// # 参数
    /// - `is_all_day`: 是否全天日程
    pub fn set_is_all_day(&mut self, is_all_day: bool) -> &mut Self {
        self.is_all_day = Some(is_all_day);
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
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

        if let Some(user_id_type) = &self.user_id_type {
            if user_id_type.trim().is_empty() {
                return Err("用户ID类型不能为空".to_string());
            }
        }

        // 验证时间逻辑
        if let (Some(start), Some(end)) = (&self.start_time, &self.end_time) {
            // 这里可以添加更复杂的时间验证逻辑
            if start.timestamp.is_some() && end.timestamp.is_some() {
                // 简单验证开始时间应该早于结束时间
                // 实际验证可能需要更复杂的逻辑
            }
        }

        Ok(())
    }
}

/// 创建日程事件响应
///
/// 包含创建成功的日程事件信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCalendarEventResponse {
    /// 日程事件信息
    pub event: Option<CalendarEvent>,
}

impl Default for CreateCalendarEventResponse {
    fn default() -> Self {
        Self {
            event: None,
        }
    }
}

impl ApiResponseTrait for CreateCalendarEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建日程事件构建器
///
/// 提供流式API来构建创建日程事件的请求。
#[derive(Debug, Clone)]
pub struct CreateCalendarEventBuilder {
    request: CreateCalendarEventRequest,
}

impl CreateCalendarEventBuilder {
    /// 创建创建日程事件构建器
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::calendar::v4::calendar_event::CreateCalendarEventBuilder;
    ///
    /// let builder = CreateCalendarEventBuilder::new("calendar_123");
    /// ```
    pub fn new(calendar_id: impl Into<String>) -> Self {
        Self {
            request: CreateCalendarEventRequest::new(calendar_id),
        }
    }

    /// 设置日程标题
    ///
    /// # 参数
    /// - `summary`: 日程标题
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::CreateCalendarEventBuilder;
    /// let builder = CreateCalendarEventBuilder::new("calendar_123")
    ///     .summary("团队会议");
    /// ```
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.request.set_summary(summary);
        self
    }

    /// 设置日程描述
    ///
    /// # 参数
    /// - `description`: 日程描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.set_description(description);
        self
    }

    /// 设置开始时间
    ///
    /// # 参数
    /// - `start_time`: 开始时间
    pub fn start_time(mut self, start_time: TimeInfo) -> Self {
        self.request.set_start_time(start_time);
        self
    }

    /// 设置结束时间
    ///
    /// # 参数
    /// - `end_time`: 结束时间
    pub fn end_time(mut self, end_time: TimeInfo) -> Self {
        self.request.set_end_time(end_time);
        self
    }

    /// 设置是否全天日程
    ///
    /// # 参数
    /// - `is_all_day`: 是否全天日程
    pub fn is_all_day(mut self, is_all_day: bool) -> Self {
        self.request.set_is_all_day(is_all_day);
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.set_user_id_type(user_id_type);
        self
    }

    /// 构建请求对象
    ///
    /// # 返回
    /// - `Ok(CreateCalendarEventRequest)`: 构建成功的请求对象
    /// - `Err(String)`: 构建失败，返回错误信息
    pub fn build(self) -> Result<CreateCalendarEventRequest, String> {
        self.request.validate()?;
        Ok(self.request)
    }

    /// 执行创建日程事件请求
    ///
    /// # 参数
    /// - `service`: 日程事件服务实例
    ///
    /// # 返回
    /// - `Ok(CreateCalendarEventResponse)`: 创建成功，返回日程事件信息
    /// - `Err(SDKError)`: 创建失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::CreateCalendarEventBuilder;
    /// use open_lark::service::calendar::v4::models::TimeInfo;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let start_time = TimeInfo {
    ///     timestamp: Some("2024-01-01T09:00:00Z".to_string()),
    ///     date: None,
    ///     timezone: Some("Asia/Shanghai".to_string()),
    /// };
    /// let end_time = TimeInfo {
    ///     timestamp: Some("2024-01-01T10:00:00Z".to_string()),
    ///     date: None,
    ///     timezone: Some("Asia/Shanghai".to_string()),
    /// };
    /// let response = CreateCalendarEventBuilder::new("calendar_123")
    ///     .summary("团队会议")
    ///     .description("讨论项目进展")
    ///     .start_time(start_time)
    ///     .end_time(end_time)
    ///     .user_id_type("open_id")
    ///     .execute(&client.calendar.v4.calendar_event)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute(self, service: &crate::service::calendar::v4::calendar_event::CalendarEventService) -> SDKResult<CreateCalendarEventResponse> {
        let request = self.build()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        service.create(&request).await
    }
}

impl Default for CreateCalendarEventBuilder {
    fn default() -> Self {
        Self {
            request: CreateCalendarEventRequest {
                calendar_id: String::new(),
                summary: None,
                description: None,
                start_time: None,
                end_time: None,
                is_all_day: None,
                recurrence: None,
                reminders: None,
                attendees: None,
                meeting_rooms: None,
                location: None,
                color: None,
                user_id_type: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::calendar::v4::models::TimeInfo;

    #[test]
    fn test_create_calendar_event_request_creation() {
        let request = CreateCalendarEventRequest::new("calendar_123");
        assert_eq!(request.calendar_id, "calendar_123");
        assert!(request.summary.is_none());
        assert!(request.description.is_none());
    }

    #[test]
    fn test_set_summary() {
        let mut request = CreateCalendarEventRequest::new("calendar_123");
        request.set_summary("团队会议");
        assert_eq!(request.summary, Some("团队会议".to_string()));
    }

    #[test]
    fn test_set_description() {
        let mut request = CreateCalendarEventRequest::new("calendar_123");
        request.set_description("讨论项目进展");
        assert_eq!(request.description, Some("讨论项目进展".to_string()));
    }

    #[test]
    fn test_set_start_time() {
        let mut request = CreateCalendarEventRequest::new("calendar_123");
        let time = TimeInfo {
            timestamp: Some("2024-01-01T09:00:00Z".to_string()),
            date: None,
            timezone: Some("Asia/Shanghai".to_string()),
        };
        request.set_start_time(time.clone());
        assert_eq!(request.start_time, Some(time));
    }

    #[test]
    fn test_set_end_time() {
        let mut request = CreateCalendarEventRequest::new("calendar_123");
        let time = TimeInfo {
            timestamp: Some("2024-01-01T10:00:00Z".to_string()),
            date: None,
            timezone: Some("Asia/Shanghai".to_string()),
        };
        request.set_end_time(time.clone());
        assert_eq!(request.end_time, Some(time));
    }

    #[test]
    fn test_set_is_all_day() {
        let mut request = CreateCalendarEventRequest::new("calendar_123");
        request.set_is_all_day(true);
        assert_eq!(request.is_all_day, Some(true));
    }

    #[test]
    fn test_set_user_id_type() {
        let mut request = CreateCalendarEventRequest::new("calendar_123");
        request.set_user_id_type("user_id");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_request_validation_success() {
        let request = CreateCalendarEventRequest::new("calendar_123");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_request_validation_empty_calendar_id() {
        let request = CreateCalendarEventRequest::new("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_request_validation_empty_user_id_type() {
        let mut request = CreateCalendarEventRequest::new("calendar_123");
        request.set_user_id_type("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_create_calendar_event_builder_creation() {
        let builder = CreateCalendarEventBuilder::new("calendar_123");
        assert_eq!(builder.request.calendar_id, "calendar_123");
        assert!(builder.request.summary.is_none());
    }

    #[test]
    fn test_builder_summary() {
        let builder = CreateCalendarEventBuilder::new("calendar_123")
            .summary("团队会议");
        assert_eq!(builder.request.summary, Some("团队会议".to_string()));
    }

    #[test]
    fn test_builder_description() {
        let builder = CreateCalendarEventBuilder::new("calendar_123")
            .description("讨论项目进展");
        assert_eq!(builder.request.description, Some("讨论项目进展".to_string()));
    }

    #[test]
    fn test_builder_is_all_day() {
        let builder = CreateCalendarEventBuilder::new("calendar_123")
            .is_all_day(true);
        assert_eq!(builder.request.is_all_day, Some(true));
    }

    #[test]
    fn test_builder_user_id_type() {
        let builder = CreateCalendarEventBuilder::new("calendar_123")
            .user_id_type("open_id");
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_builder_build_success() {
        let result = CreateCalendarEventBuilder::new("calendar_123")
            .summary("团队会议")
            .user_id_type("open_id")
            .build();
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.calendar_id, "calendar_123");
        assert_eq!(request.summary, Some("团队会议".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_builder_build_failure_empty_calendar_id() {
        let result = CreateCalendarEventBuilder::new("").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_builder_default() {
        let builder = CreateCalendarEventBuilder::default();
        assert_eq!(builder.request.calendar_id, "");
        assert!(builder.request.summary.is_none());
        assert!(builder.request.user_id_type.is_none());
    }

    #[test]
    fn test_serialization_request() {
        let request = CreateCalendarEventRequest {
            calendar_id: "calendar_123".to_string(),
            summary: Some("团队会议".to_string()),
            description: Some("讨论项目进展".to_string()),
            start_time: None,
            end_time: None,
            is_all_day: Some(false),
            recurrence: None,
            reminders: None,
            attendees: None,
            meeting_rooms: None,
            location: None,
            color: None,
            user_id_type: Some("open_id".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("calendar_123"));
        assert!(json.contains("团队会议"));
        assert!(json.contains("讨论项目进展"));
        assert!(json.contains("open_id"));
    }

    #[test]
    fn test_serialization_request_minimal() {
        let request = CreateCalendarEventRequest {
            calendar_id: "calendar_123".to_string(),
            summary: None,
            description: None,
            start_time: None,
            end_time: None,
            is_all_day: None,
            recurrence: None,
            reminders: None,
            attendees: None,
            meeting_rooms: None,
            location: None,
            color: None,
            user_id_type: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("calendar_123"));
        assert!(!json.contains("summary"));
        assert!(!json.contains("description"));
    }

    #[test]
    fn test_deserialization_response() {
        let json = r#"
        {
            "event": {
                "event_id": "event_456",
                "summary": "New Event",
                "description": "Event Description"
            }
        }
        "#;
        let response: CreateCalendarEventResponse = serde_json::from_str(json).unwrap();
        assert!(response.event.is_some());
        assert_eq!(response.event.unwrap().event_id, Some("event_456".to_string()));
    }

    #[test]
    fn test_default_response() {
        let response = CreateCalendarEventResponse::default();
        assert!(response.event.is_none());
    }

    #[test]
    fn test_request_with_whitespace_calendar_id() {
        let request = CreateCalendarEventRequest::new("   ");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_request_with_whitespace_user_id_type() {
        let mut request = CreateCalendarEventRequest::new("calendar_123");
        request.set_user_id_type("   ");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }
}